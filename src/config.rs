use crate::Repo;
use serde::{Deserialize, Serialize};
use std::error;
use std::fs;
use std::io;
use url::Url;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    #[serde(default)]
    ignore: Vec<Url>,
    #[serde(default)]
    emails: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Config, Box<dyn error::Error>> {
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

    pub fn add_ignore_url(&mut self, url: &Url) {
        if !self.ignores(&url) {
            self.ignore.push(url.clone());
        }
    }

    pub fn add_email(&mut self, email: &str) {
        let email_string = email.to_string();
        if !self.emails.contains(&email_string) {
            self.emails.push(email_string);
        }
    }

    pub fn get_emails(&self) -> Vec<(String, Option<String>)> {
        let global_email = Repo::get_global_email();

        let mut email_tuple = match global_email {
            Some(email) => vec![(format!("Always use Global <{}>", &email), None)],
            None => Vec::new(),
        };
        email_tuple.append(
            &mut self
                .emails
                .iter()
                .map(|email| (email.clone(), Some(email.clone())))
                .collect(),
        );

        email_tuple
    }

    pub fn ignores(&self, url: &Url) -> bool {
        self.ignore.contains(&url)
    }

    pub fn save(&self) -> Result<(), Box<dyn error::Error>> {
        let mut config_path = dirs::config_dir().unwrap();

        config_path.push("commit-email/commit-email.toml");

        fs::write(config_path, toml::to_string_pretty(self)?)?;

        Ok(())
    }
}
