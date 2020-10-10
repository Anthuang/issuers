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
    let issues = match opt.days {
        Some(days) => issuers::get_issues_by_days(days).await?,
        None => issuers::get_issues().await?,
    };
    for (i, issue) in issues.iter().enumerate() {
        if !issue.is_empty() {
            print!("{:?}", issue);
            if i != issues.len() - 1 {
                println!();
            }
        }
    }

    Ok(())
}
