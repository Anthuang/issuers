use crate::issues::Issue;
use color_eyre::eyre::Result;
use reqwest::header::{ETAG, IF_NONE_MATCH, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub repo: String,
    pub tag: String,
    pub etag: String,
}

impl Repo {
    pub fn new(repo: String, tag: String) -> Self {
        Self {
            repo,
            tag,
            etag: String::new(),
        }
    }

    pub async fn issues(&mut self) -> Result<Vec<Issue>> {
        let request_url = format!(
            "https://api.github.com/repos/{repo}/issues",
            repo = self.repo
        );
        let client = reqwest::Client::new();
        let response = if self.etag.is_empty() {
            client
                .get(&request_url)
                .query(&[("labels", self.tag.clone())])
                .header(USER_AGENT, "issuers")
                .send()
                .await?
        } else {
            client
                .get(&request_url)
                .query(&[("labels", self.tag.clone())])
                .header(USER_AGENT, "issuers")
                .header(IF_NONE_MATCH, self.etag.clone())
                .send()
                .await?
        };

        // Get the etag if the request was successful
        if response.status().is_success() {
            if let Some(etag) = response.headers().get(ETAG) {
                self.etag = etag.to_str()?.to_string();
            }
            Ok(response.json().await?)
        } else {
            Ok(Vec::new())
        }
    }
}
