use reqwest::header::USER_AGENT;

mod issues;

pub async fn get_issues(repo: &'static str) -> Result<issues::Issues, Box<dyn std::error::Error>> {
    let request_url = format!("https://api.github.com/repos/{repo}/issues", repo = repo);
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "issuers")
        .send()
        .await?;
    Ok(issues::Issues::new(repo, response.json().await?))
}
