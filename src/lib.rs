pub mod config;
mod errors;
pub mod history;
mod issues;
mod repo;

use chrono::prelude::*;
use chrono::Duration;
use color_eyre::eyre::Result;
use config::get_repos_from_config;
use issues::Issues;
use std::ops::Sub;

pub async fn get_issues(days: Option<i64>) -> Result<Vec<Issues>> {
    let mut result_issues = Vec::new();
    let repos = get_repos_from_config()?;

    for r in repos {
        let issues = Issues::new(r.repo.clone(), r.issues().await?);
        match days {
            Some(days) => {
                result_issues.push(
                    issues.created_after(
                        Utc::now()
                            .date()
                            .sub(Duration::days(days))
                            .and_time(NaiveTime::from_hms(0, 0, 0))
                            .expect("Failed to create date"),
                    ),
                );
            }
            None => {
                result_issues.push(issues.created_after(history::read_time()?));
            }
        }
    }

    Ok(result_issues)
}
