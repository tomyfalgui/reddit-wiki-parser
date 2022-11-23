use clap::Parser;
use reqwest;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of subreddit to parse
    #[arg(short, long, default_value_t = String::from("rust"))]
    subreddit: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Parsing {}!", args.subreddit);

    let parsed_result = fetch_subreddit_wiki(&args.subreddit)?;

    println!("{}", parsed_result);

    Ok(())
}

fn fetch_subreddit_wiki(subreddit: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(format!(
        "https://old.reddit.com/r/{}/wiki/index",
        subreddit
    ))?
    .text()?;

    Ok(resp)
}
