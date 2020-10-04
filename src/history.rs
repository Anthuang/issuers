use crate::errors::HistoryError;
use crate::repo::Repo;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use toml::Value;

const HISTORY_FILE: &str = "/tmp/issuers_history";

#[derive(Serialize, Deserialize)]
pub struct History {
    pub last_changed: DateTime<Utc>,
    repos: Vec<Repo>,
}

impl Default for History {
    fn default() -> Self {
        History::new()
    }
}

impl History {
    pub fn new() -> Self {
        // If history file does not exist, then use the Unix Epoch (all issues
        // would be considered new) and create the file.
        if fs::metadata(HISTORY_FILE).is_err() {
            return Self {
                last_changed: Utc.timestamp(0, 0),
                repos: Vec::new(),
            };
        }

        let toml: String =
            String::from_utf8_lossy(&fs::read(HISTORY_FILE).expect("Could not read history file"))
                .parse()
                .expect("String parse failed");
        toml::from_str(&toml).expect("Could not parse toml file")
    }

    /// Updates the passed in repos read from the config file. This attaches
    /// information to the repos from the history file, such as etag.
    pub fn update_repos(&self, repos: &mut Vec<Repo>) -> Result<(), HistoryError> {
        if self.repos.is_empty() {
            return Ok(());
        }

        let mut repo_map = HashMap::new();
        for r in self.repos.iter() {
            repo_map.insert(r.repo.clone(), r.etag.clone());
        }
        for r in repos {
            if repo_map.contains_key(r.repo.as_str()) {
                r.etag = repo_map[&r.repo].clone();
            }
        }
        Ok(())
    }

    pub fn write(&mut self, repos: Vec<Repo>) -> Result<(), HistoryError> {
        self.last_changed = Utc::now();
        self.repos = repos;

        let toml = toml::to_string(&Value::try_from(self)?)?;
        fs::write(HISTORY_FILE, toml)?;
        Ok(())
    }
}
