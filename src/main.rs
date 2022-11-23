use clap::Parser;
use reqwest;
use tl;

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

    let r_formatted = format!(r"{}", parsed_result);
    let dom = tl::parse(&r_formatted, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    dom.query_selector(".wiki-page-content")
        .expect("Failed to find container")
        .for_each(move |el| {
            let element = el.get(parser).unwrap();
            println!("{}", element.inner_text(parser).into_owned());
        });

    Ok(())
}

fn fetch_subreddit_wiki(subreddit: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp =
        reqwest::blocking::get(format!("https://old.reddit.com/r/{}/wiki/index", subreddit))?
            .text()?;

    Ok(resp)
}
