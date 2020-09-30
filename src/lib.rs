use anyhow::Result;
use reqwest::header::USER_AGENT;

pub mod config;
mod errors;
pub mod history;
mod issues;

use config::get_repos;
use issues::Issues;

pub async fn get_issues() -> Result<Vec<Issues>> {
    let mut result_issues = Vec::new();
    let repos = get_repos();

    for r in repos {
        let request_url = format!("https://api.github.com/repos/{repo}/issues", repo = r.0);
        let client = reqwest::Client::new();
        let response = client
            .get(&request_url)
            .query(&[("labels", r.1.clone())])
            .header(USER_AGENT, "issuers")
            .send()
            .await?;
        let issues = Issues::new(r.0, response.json().await?);
        result_issues.push(issues.created_after(history::read_time()?).with_tag(r.1));
    }
    Ok(result_issues)
}
