use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::fmt;

#[derive(Deserialize, Serialize, Clone)]
pub struct Issues {
    repo: String,
    issues: Vec<Issue>,
}

impl Issues {
    pub fn new(repo: String, issues: Vec<Issue>) -> Issues {
        Self { repo, issues }
    }

    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn with_tag(&self, tag: &str) -> Issues {
        Issues::new(
            self.repo.clone(),
            self.issues
                .iter()
                .filter(|i| i.has_tag(tag))
                .cloned()
                .collect(),
        )
    }

    pub fn created_after(&self, time: DateTime<Utc>) -> Issues {
        Issues::new(
            self.repo.clone(),
            self.issues
                .iter()
                .filter(|i| i.created_at >= time)
                .cloned()
                .collect(),
        )
    }
}

impl fmt::Debug for Issues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.issues.is_empty() {
            return Ok(());
        }

        writeln!(f, "{repo}:", repo = self.repo).expect("Print failed");
        for (i, issue) in self.issues.iter().enumerate() {
            writeln!(
                f,
                "{index}: {title}: {url}",
                index = i + 1,
                title = issue.title,
                url = issue.url
            )
            .expect("Print failed");
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Issue {
    title: String,
    #[serde(rename = "html_url")]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Label {
    name: String,
}
