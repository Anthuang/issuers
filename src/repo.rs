use crate::issues::Issue;
use anyhow::Result;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub repo: String,
    pub tag: String,
}

impl Repo {
    pub async fn issues(&self) -> Result<Vec<Issue>> {
        let request_url = format!(
            "https://api.github.com/repos/{repo}/issues",
            repo = self.repo
        );
        let client = reqwest::Client::new();
        let response = client
            .get(&request_url)
            .query(&[("labels", self.tag.clone())])
            .header(USER_AGENT, "issuers")
            .send()
            .await?;
        Ok(response.json().await?)
    }
}
