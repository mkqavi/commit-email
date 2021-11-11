use git2::{Config, ConfigLevel, Error, Repository};
use repo_cli::ScpPath;
use std::path::Path;
use url::Url;

pub struct Repo {
    repository: Repository,
}

impl Repo {
    pub fn new(path: &Path) -> Result<Repo, Error> {
        Ok(Repo {
            repository: Repository::open(path)?,
        })
    }

    pub fn get_global_email() -> Option<String> {
        let config = Config::open_default().ok()?;

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

    pub fn set_local_email(&self, email: &str) -> Result<(), Error> {
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

        Repo::parse_url_string(remote.url()?)
    }

    fn parse_url_string(url_string: &str) -> Option<Url> {
        match Url::parse(url_string) {
            Ok(url) => Some(url),
            Err(_) => match ScpPath::parse(url_string) {
                Ok(scp_path) => Some(scp_path.to_url()),
                Err(_) => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_string_works_with_scp_style() {
        let url = Repo::parse_url_string("git@github.com:mkqavi/commit-email.git");
        assert_eq!(
            url,
            Url::parse("ssh://git@github.com/mkqavi/commit-email").ok()
        );
    }

    #[test]
    fn parse_url_string_works_with_normal_url() {
        let url = Repo::parse_url_string("git://github.com/robbyrussell/oh-my-zsh.git");
        assert_eq!(
            url,
            Url::parse("git://github.com/robbyrussell/oh-my-zsh.git").ok()
        );
    }

    #[test]
    fn parse_url_string_works_with_no_url() {
        let url = Repo::parse_url_string("No/Url");
        assert_eq!(url, None);
    }
}
