pub mod config;
mod errors;
pub mod history;
mod issues;
mod repo;

use chrono::prelude::*;
use chrono::Duration;
use color_eyre::eyre::Result;
use config::get_repos_from_config;
use history::History;
use issues::Issues;
use std::ops::Sub;

pub async fn get_issues(days: Option<i64>) -> Result<Vec<Issues>> {
    let mut result_issues = Vec::new();
    let mut history = History::new();
    let mut repos = get_repos_from_config()?;
    history.update_repos(&mut repos)?;

    for r in &mut repos {
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
                result_issues.push(issues.created_after(history.last_changed));
            }
        }
    }

    history.write(repos)?;
    Ok(result_issues)
}
