use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::fmt;

#[derive(Deserialize, Serialize, Clone, PartialEq)]
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

    pub fn with_tag(&self, tag: String) -> Issues {
        Issues::new(
            self.repo.clone(),
            self.issues
                .iter()
                .filter(|i| i.has_tag(tag.clone()))
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Issue {
    title: String,
    #[serde(rename = "html_url")]
    url: String,
    labels: Vec<Label>,
    created_at: DateTime<Utc>,
}

impl Issue {
    fn has_tag(&self, tag: String) -> bool {
        for label in self.labels.iter() {
            if label.name == tag {
                return true;
            }
        }
        false
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct Label {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn label(name: &str) -> Label {
        Label {
            name: name.to_string(),
        }
    }

    fn issue_with_tags(tag: Vec<Label>) -> Issue {
        Issue {
            title: "".to_string(),
            url: "".to_string(),
            labels: tag,
            created_at: Utc.timestamp(0, 0),
        }
    }

    fn issue_with_timestamp(sec: i64) -> Issue {
        Issue {
            title: "".to_string(),
            url: "".to_string(),
            labels: Vec::new(),
            created_at: Utc.timestamp(sec, 0),
        }
    }

    #[test]
    fn test_issue_has_tag() {
        assert!(issue_with_tags(vec![label("1")]).has_tag("1".to_string()));
        assert!(issue_with_tags(vec![label("1"), label("2"), label("3"),]).has_tag("1".to_string()));
        assert!(!issue_with_tags(vec![label("1")]).has_tag("2".to_string()));
    }

    #[test]
    fn test_issues_empty() {
        let issues = Issues::new("test".to_string(), Vec::new());
        assert!(issues.is_empty());
        assert_eq!(issues.with_tag("test".to_string()), issues);
        assert_eq!(issues.created_after(Utc.timestamp(0, 0)), issues);
    }

    #[test]
    fn test_issues_with_tag() {
        let issues = Issues::new(
            "test".to_string(),
            vec![
                issue_with_tags(vec![label("1")]),
                issue_with_tags(vec![label("2")]),
                issue_with_tags(vec![label("1"), label("3")]),
            ],
        );
        assert_eq!(
            issues.with_tag("1".to_string()),
            Issues::new(
                "test".to_string(),
                vec![
                    issue_with_tags(vec![label("1")]),
                    issue_with_tags(vec![label("1"), label("3")]),
                ],
            )
        );
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
