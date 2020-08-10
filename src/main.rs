use chrono::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let issues = issuers::get_issues("nushell/nushell").await?;
    print!(
        "{:?}",
        issues
            .created_after("2020-06-01T23:56:58Z".parse::<DateTime<Utc>>().unwrap())
            .with_tag("good first issue")
    );
    Ok(())
}
