use std::{env, fs};

use anyhow::Result;
use serde::Deserialize;
use toml::Value;

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub repo: String,
    pub tag: String,
}

pub fn get_repos() -> Result<Vec<Repo>> {
    let mut result_vec = Vec::new();
    let home = env::var("HOME")?;
    let file_path = format!("{}/.issuers.toml", home);
    let file = match fs::read(&file_path) {
        Ok(file) => file,
        Err(_) => {
            fs::File::create(file_path)?;
            return Ok(Vec::new());
        }
    };
    let config_toml = String::from_utf8_lossy(&file).parse::<Value>()?;
    let config_table = config_toml
        .as_table()
        .expect("Config formatted incorrectly");
    for k in config_table.iter() {
        let repo = k.1.clone().try_into::<Repo>()?;
        result_vec.push(repo);
    }
    Ok(result_vec)
}
