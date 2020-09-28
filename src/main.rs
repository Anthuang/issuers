use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let issues = issuers::get_issues().await?;
    for i in issues.iter() {
        if !i.is_empty() {
            println!("{:?}", i);
        }
    }

    issuers::history::write(issues)?;
    Ok(())
}
