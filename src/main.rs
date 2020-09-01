use cursive::views::{Dialog, SelectView};
use cursive::Cursive;
use git2::{ConfigLevel, Repository};
use serde::{Deserialize, Serialize};
use std::env;
use std::error;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Config {
    #[serde(default)]
    ignore: Vec<PathBuf>,
    #[serde(default)]
    emails: Vec<String>,
}

impl Config {
    fn load() -> Result<Config, Box<dyn error::Error>> {
        let mut config_dir = match dirs::config_dir() {
            Some(path) => path,
            None => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Could not find the config directory",
                )))
            }
        };

        config_dir.push("commit-email");

        if fs::read_dir(&config_dir).is_err() {
            fs::create_dir(&config_dir)?;
        }

        let mut config_file = config_dir;
        config_file.push("commit-email.toml");

        let config_string = match fs::read_to_string(&config_file) {
            Err(_) => {
                fs::File::create(&config_file)?;
                "".to_string()
            }
            Ok(content) => content,
        };

        let config: Config = match toml::from_str(&config_string) {
            Ok(config) => config,
            Err(_) => Config::default(),
        };

        Ok(config)
    }

    fn add_path(&mut self, path: &PathBuf) {
        if !self.ignores(&path) {
            self.ignore.push(path.clone());
        }
    }

    fn add_email(&mut self, email: &str) {
        let email_string = email.to_string();
        if !self.emails.contains(&email_string) {
            self.emails.push(email_string);
        }
    }

    fn get_emails(&self) -> Vec<(String, Option<String>)> {
        let global_email = match get_global_email() {
            Ok(email) => email,
            Err(error) => panic!("{}", error),
        };

        let mut email_tuple = vec![(format!("Always use Global <{}>", &global_email), None)];
        email_tuple.append(
            &mut self
                .emails
                .iter()
                .map(|email| (email.clone(), Some(email.clone())))
                .collect(),
        );

        email_tuple
    }

    fn ignores(&self, path: &PathBuf) -> bool {
        self.ignore.contains(&path)
    }

    fn save(&self) -> Result<(), Box<dyn error::Error>> {
        let mut config_path = dirs::config_dir().unwrap();

        config_path.push("commit-email/commit-email.toml");

        fs::write(config_path, toml::to_string_pretty(self)?)?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::load()?;
    let current_dir = env::current_dir()?;

    if config.ignores(&current_dir) {
        return Ok(());
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
        submit_email(ui, email, &mut config.clone(), current_dir.clone())
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

fn get_global_email() -> Result<String, Box<dyn error::Error>> {
    let config = git2::Config::open_default()?;

    Ok(config.get_string("user.email")?)
}

fn submit_email(
    ui: &mut Cursive,
    email: &Option<String>,
    config: &mut Config,
    current_dir: PathBuf,
) {
    ui.quit();

    match email {
        Some(email) => {
            config.add_email(email);

            // Write to .git/config
            println!("Writing {} to .git/config", email);
        }
        None => config.add_path(&current_dir),
    }

    config.save().unwrap();
}
