use config::{Config, File};
use git2::{ConfigLevel, Repository};
use std::env;
use std::error;
use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = read_config()?;

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

    if fs::read(&config_file).is_err() {
        fs::File::create(&config_file)?;
    }

    let mut config = Config::default();
    config.merge(File::from(config_file))?;

    Ok(config)
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
