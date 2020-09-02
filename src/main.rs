use config::Config;
use cursive::views::{Dialog, SelectView};
use cursive::Cursive;
use repo::Repo;
use std::env;
use std::error;
use url::Url;

mod config;
mod repo;

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::load()?;
    let repo = Repo::new(&env::current_dir()?)?;
    let config_url = repo.get_remote_url();

    if let Some(url) = &config_url {
        if config.ignores(&url) {
            return Ok(());
        }
    }

    if repo.get_local_email().is_some() {
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
