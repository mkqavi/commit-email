use git2::{ConfigLevel, Repository};
use repo_cli::ScpPath;
use std::path::PathBuf;
use url::Url;

pub struct Repo {
    repository: Repository,
}

impl Repo {
    pub fn new(path: &PathBuf) -> Result<Repo, git2::Error> {
        Ok(Repo {
            repository: Repository::open(path)?,
        })
    }

    pub fn get_global_email() -> Option<String> {
        let config = git2::Config::open_default().ok()?;

        config.get_string("user.email").ok()
    }

    pub fn get_local_email(&self) -> Option<String> {
        let config = self.repository.config().ok()?;
        let email_entry = config.get_entry("user.email").ok()?;

        match email_entry.level() {
            ConfigLevel::Local => email_entry.value().map(|s| s.to_string()),
            _ => None,
        }
    }

    pub fn set_local_email(&self, email: &str) -> Result<(), git2::Error> {
        self.repository
            .config()?
            .open_level(ConfigLevel::Local)?
            .set_str("user.email", email)
    }

    pub fn get_remote_url(&self) -> Option<Url> {
        let remote = match self.repository.find_remote("origin") {
            Ok(remote) => remote,
            Err(_) => match self.repository.remotes().ok()?.get(0) {
                Some(remote_name) => self.repository.find_remote(remote_name).ok()?,
                None => return None,
            },
        };

        let url_string = remote.url()?;

        match ScpPath::parse(url_string) {
            Ok(scp_path) => Some(scp_path.to_url()),
            Err(_) => Url::parse(url_string).ok(),
        }
    }
}
