use config::Config;
use cursive::views::{Dialog, SelectView};
use cursive::Cursive;
use git2::{ConfigLevel, Repository};
use repo_cli::ScpPath;
use std::env;
use std::error;
use std::path::PathBuf;
use url::Url;

mod config;

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::load()?;
    let current_dir = env::current_dir()?;
    let config_url = get_repository_url(&current_dir);

    if let Some(url) = &config_url {
        if config.ignores(&url) {
            return Ok(());
        }
    }

    if get_repository_email(&current_dir)?.is_some() {
        return Ok(());
    }

    // Create UI
    let mut ui = cursive::default();
    ui.load_toml(include_str!("../assets/style.toml")).unwrap();

    let mut sv = SelectView::new();
    sv.add_all(config.get_emails());
    sv.set_on_submit(move |ui, email| {
        submit_email(ui, email, &mut config.clone(), config_url.clone())
    });

    ui.add_layer(Dialog::around(sv).title("Please select an email for your commit"));

    ui.run();

    Ok(())
}

fn get_repository_email(current_dir: &PathBuf) -> Result<Option<String>, Box<dyn error::Error>> {
    let repo = Repository::open(current_dir)?;

    let config = repo.config()?;

    let email_entry = match config.get_entry("user.email") {
        Ok(entry) => entry,
        Err(error) => panic!("{}", error),
    };

    match email_entry.level() {
        ConfigLevel::Local => Ok(Some(email_entry.value().unwrap().to_string())),
        _ => Ok(None),
    }
}

fn get_repository_url(current_dir: &PathBuf) -> Option<Url> {
    let repo = Repository::open(current_dir).ok()?;

    let remote = match repo.find_remote("origin") {
        Ok(remote) => remote,
        Err(_) => match repo.remotes().ok()?.get(0) {
            Some(remote_name) => repo.find_remote(remote_name).ok()?,
            None => return None,
        },
    };

    let url_string = remote.url()?;

    match ScpPath::parse(url_string) {
        Ok(scp_path) => Some(scp_path.to_url()),
        Err(_) => Url::parse(url_string).ok(),
    }
}

fn get_global_email() -> Result<String, Box<dyn error::Error>> {
    let config = git2::Config::open_default()?;

    Ok(config.get_string("user.email")?)
}

fn submit_email(
    ui: &mut Cursive,
    email: &Option<String>,
    config: &mut Config,
    config_url: Option<Url>,
) {
    ui.quit();

    match email {
        Some(email) => {
            config.add_email(email);

            // Write to .git/config
            println!("Writing {} to .git/config", email);
        }
        None => {
            if let Some(url) = &config_url {
                config.add_ignore_url(&url);
            }
        }
    }

    config.save().unwrap();
}
