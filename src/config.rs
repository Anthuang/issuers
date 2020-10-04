use std::{env, fs};

use crate::repo::Repo;
use color_eyre::eyre::Result;
use toml::Value;

pub fn get_repos_from_config() -> Result<Vec<Repo>> {
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
        let repo = Repo::new(
            k.1["repo"]
                .as_str()
                .expect("Repo must exist in repo config")
                .to_string(),
            k.1["tag"]
                .as_str()
                .expect("Repo must exist in repo config")
                .to_string(),
        );
        result_vec.push(repo);
    }
    Ok(result_vec)
}
