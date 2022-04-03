use std::env::{var};
use std::error::Error;

use clap::Parser;
use serde::{Serialize, Deserialize};
use serde_json;
use reqwest::StatusCode;
use reqwest::header::{CONTENT_TYPE, AUTHORIZATION, USER_AGENT};

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
    CliArgs::parse();
}
