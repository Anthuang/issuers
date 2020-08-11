use reqwest::header::USER_AGENT;

mod errors;
pub mod history;
mod issues;

use errors::IssuesError;
use issues::Issues;

pub async fn get_issues(repo: String) -> Result<Issues, IssuesError> {
    let request_url = format!("https://api.github.com/repos/{repo}/issues", repo = repo);
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "issuers")
        .send()
        .await?;
    let issues = Issues::new(repo, response.json().await?);
    Ok(issues)
}
