use color_eyre::eyre::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "issuers")]
struct Opt {
    /// Number of past days to search for issues
    #[structopt(short = "d", long)]
    days: Option<i64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::from_args();
    let issues = issuers::get_issues(opt.days).await?;
    for i in issues.iter() {
        if !i.is_empty() {
            println!("{:?}", i);
        }
    }
    issuers::history::write(issues)?;
    Ok(())
}
