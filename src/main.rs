use std::env::{var};
use std::error::Error;
use reqwest::header::{CONTENT_TYPE, AUTHORIZATION, USER_AGENT};
use reqwest::StatusCode;
use base64;
use serde::{Serialize, Deserialize};
use serde_json;

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
    expires_in: u32,
    scope: String,
    token_type: String
}

fn parse_token_response(response: reqwest::blocking::Response) -> Result<RedditTokenResponse, Box<dyn Error>> {
    match serde_json::from_value(response.json().unwrap()) {
        Ok(valid_token) => Ok(valid_token),
        Err(e) => Err(e.into())
    }
}

fn get_reddit_token(username: &str, password: &str, app_id: &str, app_secret: &str) -> Result<RedditTokenResponse, Box<dyn Error>> {
    let auth = format!("{}:{}", app_id, app_secret);
    let authb64 = format!("Basic {}", base64::encode(auth));

    let mut map = std::collections::HashMap::new();
    map.insert("grant_type", "password");
    map.insert("username", &username);
    map.insert("password", &password);

    let client = reqwest::blocking::Client::new();
    let response = client.post("https://www.reddit.com/api/v1/access_token")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(USER_AGENT, "backuppit/0.0.1 by u/rrrigggbbby")
        .header(AUTHORIZATION, &authb64)
        .form(&map)
        .send()?;

    if response.status() == StatusCode::OK {
        // For some reason, some error responses like "invalid_grant" returns inside a OK HTTP response
        // so we have to double check if the response is the right JSON
        match parse_token_response(response) {
            Ok(valid_token) => Ok(valid_token),
            Err(e) => Err(e.into())
        }
    } else {
        // TODO: Handle return codes
        // 401 Unauthorized
        // 400 Bad Request
        Err("Something went wrong requesting your tokens".into())
    }
}

fn main() {
    CliArgs::parse();
    dotenv().ok();

    let reddit_secret = var("REDDIT_SECRET")
        .expect("Couldn't find your Reddit API secret, are you sure your .env has a REDDIT_SECRET key?");
    let username = var("REDDIT_USERNAME")
        .expect("Couldn't find your Reddit username, are you sure your .env has a REDDIT_SECRET key?");
    let password = var("REDDIT_PASSWORD")
        .expect("Couldn't find your Reddit password, are you sure your .env has a REDDIT_SECRET key?");
    let reddit_app_id = var("REDDIT_APP_ID")
        .expect("Couldn't find your Reddit App ID, are you sure your .env has a REDDIT_SECRET key?");

    let token = get_reddit_token(&username, &password, &reddit_app_id, &reddit_secret)
        .expect("Error obtaining access token");
}
