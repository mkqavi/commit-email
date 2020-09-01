use git2::{ConfigLevel, Repository};
use serde::{Deserialize, Serialize};
use std::env;
use std::error;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    ignore: Vec<PathBuf>,
}

impl Config {
    fn add_path(&mut self, path: &PathBuf) {
        if !self.is_ignored(&path) {
            self.ignore.push(path.clone());
        }
    }

    fn is_ignored(&self, path: &PathBuf) -> bool {
        self.ignore.contains(&path)
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut config = read_config()?;

    config.add_path(&PathBuf::from("/home/mkqavi"));

    if config.is_ignored(&PathBuf::from("/home/mkqavi")) {
        println!("ignored");
    }

    println!("{:?}", config);

    save_config(&config)?;

    print_user_email()?;

    Ok(())
}

fn read_config() -> Result<Config, Box<dyn error::Error>> {
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

fn save_config(config: &Config) -> Result<(), Box<dyn error::Error>> {
    let mut config_path = dirs::config_dir().unwrap();

    config_path.push("commit-email/commit-email.toml");

    fs::write(config_path, toml::to_string_pretty(config)?)?;

    Ok(())
}

fn print_user_email() -> Result<(), Box<dyn error::Error>> {
    let current_dir = env::current_dir()?;

    let repo = Repository::open(current_dir)?;

    let config = repo.config()?;

    let email_entry = match config.get_entry("user.email") {
        Ok(entry) => entry,
        Err(error) => panic!("{}", error),
    };

    println!(
        "{}: {}",
        match email_entry.level() {
            ConfigLevel::Local => {
                "Local"
            }
            ConfigLevel::Global => {
                "Global"
            }
            _ => {
                "Other"
            }
        },
        email_entry.value().unwrap()
    );

    Ok(())
}
