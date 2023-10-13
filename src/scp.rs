/**
 * Copyright 2021 EdenEast
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScpPath {
    pub host: String,
    pub username: String,
    pub path: String,
}

impl ScpPath {
    pub fn parse(s: &str) -> Result<Self> {
        s.parse()
    }
}

impl FromStr for ScpPath {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // Example of regex construction: https://regex101.com/r/elsHDo/1
        let regex = Regex::new(r"^((?:[^@]+@)?)([^:]+):/?(.+)$")?;

        let captures = regex
            .captures(s)
            .ok_or_else(|| anyhow!("url: {} does not match scp regex", s))?;

        let username = captures
            .get(1)
            .map(|s| s.as_str())
            .map(|s| s.trim_end_matches('@'))
            .unwrap_or("git")
            .to_owned();

        let host = captures.get(2).unwrap().as_str().to_owned();
        let path = captures
            .get(3)
            .unwrap()
            .as_str()
            .trim_end_matches(".git")
            .to_owned();

        Ok(Self {
            host,
            username,
            path,
        })
    }
}

impl ScpPath {
    pub fn to_url(&self) -> Url {
        let str = format!("ssh://{}@{}/{}", self.username, self.host, self.path);
        Url::parse(&str).unwrap()
    }
}

#[allow(clippy::from_over_into)]
impl Into<Url> for ScpPath {
    fn into(self) -> Url {
        self.to_url()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn git_ssh() {
        let scp = ScpPath::from_str("git@github.com:edeneast/repo").unwrap();

        assert_eq!(scp.username, "git");
        assert_eq!(scp.host, "github.com");
        assert_eq!(scp.path, "edeneast/repo");
    }

    #[test]
    fn to_url() {
        let scp = ScpPath::from_str("git@github.com:edeneast/repo").unwrap();
        let url = scp.to_url();

        assert_eq!(url.as_str(), "ssh://git@github.com/edeneast/repo");
    }
}
