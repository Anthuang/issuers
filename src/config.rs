use std::{env, fs};

use toml::Value;

pub fn get_repos() -> Vec<(String, String)> {
    let mut result_vec = Vec::new();
    let home = env::var("HOME").expect("HOME env variable not set");
    let file_path = format!("{}/.issuers.toml", home);
    let file = match fs::read(&file_path) {
        Ok(file) => file,
        Err(_) => {
            fs::File::create(file_path).expect("Could not create config file");
            return Vec::new();
        }
    };
    let config_toml = String::from_utf8_lossy(&file)
        .parse::<Value>()
        .expect("String parse failed");
    let config_table = config_toml
        .as_table()
        .expect("Config formatted incorrectly");
    for k in config_table.iter() {
        result_vec.push((
            k.1["repo"]
                .as_str()
                .expect("Repo must exist in repo config")
                .to_string(),
            k.1["tag"]
                .as_str()
                .expect("Tag must exist in repo config")
                .to_string(),
        ));
    }
    result_vec
}
