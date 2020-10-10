use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::fmt;

/// A respository's URL and its issues.
#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Issues {
    repo: String,
    issues: Vec<Issue>,
}

impl Issues {
    pub fn new(repo: String, issues: Vec<Issue>) -> Self {
        Self { repo, issues }
    }

    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn created_after(&self, time: DateTime<Utc>) -> Self {
        Self::new(
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
                "{index}: \"{title}\": {url}",
                index = i + 1,
                title = issue.title,
                url = issue.url
            )
            .expect("Print failed");
        }
        Ok(())
    }
}

/// Representation of a Github issue.
///
/// Used to deserialize the Github Issues API response. See
/// https://developer.github.com/v3/issues.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Issue {
    title: String,
    #[serde(rename = "html_url")]
    url: String,
    labels: Vec<Label>,
    created_at: DateTime<Utc>,
}

/// Representation of a Github tag.
///
/// Used to deserialize the Github Issues API response. See
/// https://developer.github.com/v3/issues.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct Label {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn issue_with_timestamp(sec: i64) -> Issue {
        Issue {
            title: String::new(),
            url: String::new(),
            labels: Vec::new(),
            created_at: Utc.timestamp(sec, 0),
        }
    }

    #[test]
    fn test_issues_empty() {
        let issues = Issues::new("test".to_string(), Vec::new());
        assert!(issues.is_empty());
        assert_eq!(issues.created_after(Utc.timestamp(0, 0)), issues);
    }

    #[test]
    fn test_issues_created_after() {
        let issues = Issues::new(
            "test".to_string(),
            vec![
                issue_with_timestamp(1),
                issue_with_timestamp(2),
                issue_with_timestamp(3),
            ],
        );
        assert_eq!(
            issues.created_after(Utc.timestamp(2, 0)),
            Issues::new(
                "test".to_string(),
                vec![issue_with_timestamp(2), issue_with_timestamp(3)],
            )
        )
    }
}
