use dotenv::dotenv;
use clap::Parser;
use std::env::{var};

#[derive(Parser, Debug)]
struct CliArgs {
    /// Reddit's post ID
    #[clap(long = "id")]
    post_id: String,
    /// Path to save the post
    #[clap(short, long, parse(from_os_str))]
    output: std::path::PathBuf
}

fn main() {
    dotenv().ok();

    let reddit_secret = var("REDDIT_SECRET")
        .expect("Couldn't find your Reddit API secret, are you sure your .env has a REDDIT_SECRET key?");
    let args = CliArgs::parse();

    println!("{:?}", args);
}
