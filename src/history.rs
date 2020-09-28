use crate::errors::IssuersError;
use crate::issues::Issues;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use toml::Value;

const HISTORY_FILE: &str = "/tmp/issuers_history";

#[derive(Serialize, Deserialize)]
struct History {
    last_changed: DateTime<Utc>,
    issues: Vec<Issues>,
}

pub fn read_time() -> Result<DateTime<Utc>, IssuersError> {
    // If history file does not exist, then use the Unix Epoch (all issues would
    // be considered new) and create the file.
    if fs::metadata(HISTORY_FILE).is_err() {
        fs::File::create(HISTORY_FILE)?;

        let ts = Utc.timestamp(0, 0);
        let history = History {
            last_changed: ts,
            issues: Vec::new(),
        };

        let toml = toml::to_string(&Value::try_from(&history)?)?;
        fs::write(HISTORY_FILE, toml)?;
        return Ok(ts);
    }

    let toml: String = String::from_utf8_lossy(&fs::read(HISTORY_FILE)?)
        .parse()
        .expect("String parse failed");
    let history: History = toml::from_str(&toml)?;
    Ok(history.last_changed)
}

pub fn write(issues: Vec<Issues>) -> Result<(), IssuersError> {
    let history = History {
        last_changed: Utc::now(),
        issues,
    };

    let toml = toml::to_string(&Value::try_from(&history)?)?;
    fs::write(HISTORY_FILE, toml)?;
    Ok(())
}
