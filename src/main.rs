use std::env::{var};
use std::error::Error;
use reqwest::header::{CONTENT_TYPE, AUTHORIZATION, HeaderValue, HeaderMap};
use reqwest::StatusCode;
use base64;
use serde::{Serialize, Deserialize};

use dotenv::dotenv;
use clap::Parser;

#[derive(Parser, Debug)]
struct CliArgs {
    /// Reddit's post ID
    #[clap(long = "id")]
    post_id: String,
    /// Path to save the post
    #[clap(short, long, parse(from_os_str))]
    output: std::path::PathBuf
}

#[derive(Serialize, Deserialize, Debug)]
struct RedditTokenResponse {
    access_token: String,
    expires_in: u8,
    scope: String,
    token_type: String
}

fn print_response(response: reqwest::blocking::Response) {
    let json: RedditTokenResponse = response.json().unwrap();
    //println!("{}\n{}\n{}\n{}", json.access_token, json.expires_in, json.scope, json.token_type);
    println!("{:?}", json);
}

fn get_reddit_token(username: &str, password: &str, app_id: &str, app_secret: &str) -> Result<String, Box<dyn Error>> {
    let auth = format!("{}:{}", app_id, app_secret);
    let authb64 = base64::encode(auth);

    let client = reqwest::blocking::Client::new();
    // let mut headers = HeaderMap::new();
    // headers.append(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
    // headers.append(AUTHORIZATION, HeaderValue::from_static(&authb64));

    let response = client.post("https://www.reddit.com/api/v1/access_token")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(AUTHORIZATION, &authb64)
        .body(format!("grant_type=password&username={}&password={}", username, password))
        .send()?;

    match response.status() {
        StatusCode::OK => print_response(response),
        _ => panic!("{}", response.status())
    };

    Ok(String::from("jóia"))
}

fn main() {
    dotenv().ok();

    let reddit_secret = var("REDDIT_SECRET")
        .expect("Couldn't find your Reddit API secret, are you sure your .env has a REDDIT_SECRET key?");
    let username = var("REDDIT_USERNAME")
        .expect("Couldn't find your Reddit username, are you sure your .env has a REDDIT_SECRET key?");
    let password = var("REDDIT_PASSWORD")
        .expect("Couldn't find your Reddit password, are you sure your .env has a REDDIT_SECRET key?");
    let reddit_app_id = var("REDDIT_APP_ID")
        .expect("Couldn't find your Reddit App ID, are you sure your .env has a REDDIT_SECRET key?");
    let args = CliArgs::parse();

    // println!("{:?}", args);
    get_reddit_token(&username, &password, &reddit_app_id, &reddit_secret);
}
