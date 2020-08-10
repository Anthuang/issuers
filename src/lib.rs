use chrono::prelude::*;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::clone::Clone;

#[derive(Debug)]
pub struct Issues {
    issues: Vec<Issue>,
}

impl Issues {
    fn new(issues: Vec<Issue>) -> Issues {
        Self { issues }
    }

    pub fn with_tag(&self, tag: &str) -> Issues {
        Issues::new(
            self.issues
                .iter()
                .filter(|i| i.has_tag(tag))
                .cloned()
                .collect(),
        )
    }

    pub fn created_after(&self, time: DateTime<Utc>) -> Issues {
        Issues::new(
            self.issues
                .iter()
                .filter(|i| i.created_at >= time)
                .cloned()
                .collect(),
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Issue {
    title: String,
    url: String,
    labels: Vec<Label>,
    created_at: DateTime<Utc>,
}

impl Issue {
    fn has_tag(&self, tag: &str) -> bool {
        for label in self.labels.iter() {
            if label.name == tag {
                return true;
            }
        }
        false
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Label {
    name: String,
}

pub async fn get_issues(repo: &str) -> Result<Issues, Box<dyn std::error::Error>> {
    let request_url = format!("https://api.github.com/repos/{repo}/issues", repo = repo);
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "issuers")
        .send()
        .await?;
    Ok(Issues::new(response.json().await?))
}
