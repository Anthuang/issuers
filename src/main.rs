use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let issues = issuers::get_issues(String::from("nushell/nushell")).await?;
    print!(
        "{:?}",
        issues
            .created_after(issuers::history::read_time()?)
            .with_tag("good first issue")
    );
    issuers::history::write(issues)?;
    Ok(())
}
