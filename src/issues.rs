use chrono::prelude::*;
use serde::Deserialize;
use std::clone::Clone;
use std::fmt;

pub struct Issues {
    repo: &'static str,
    issues: Vec<Issue>,
}

impl Issues {
    pub fn new(repo: &'static str, issues: Vec<Issue>) -> Issues {
        Self { repo, issues }
    }

    pub fn with_tag(&self, tag: &str) -> Issues {
        Issues::new(
            self.repo,
            self.issues
                .iter()
                .filter(|i| i.has_tag(tag))
                .cloned()
                .collect(),
        )
    }

    pub fn created_after(&self, time: DateTime<Utc>) -> Issues {
        Issues::new(
            self.repo,
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
