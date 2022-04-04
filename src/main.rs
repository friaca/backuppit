use std::fs::{File};
use std::io::Write;
use std::error::Error;

use clap::Parser;
use regex::Regex;
// use serde::{Serialize, Deserialize};
// use serde_json;
// use reqwest::StatusCode;
use reqwest::header::{USER_AGENT};

#[derive(Parser, Debug)]
struct CliArgs {
    /// Reddit's post ID, can be a full URL or a fullname 6 characters long ID
    #[clap(long = "id")]
    post_id: String,
    /// Path to save the post
    #[clap(short, long, parse(from_os_str))]
    output: std::path::PathBuf
}

fn try_extract_post_id(input: &str) -> Option<String> {
    let regexes = [
        /* Singular ID */ 
        Regex::new(r"^([\w\d]{6})$").unwrap(), 
        /* Half URL */ 
        Regex::new(r"^https://www\.reddit\.com/([\w\d]{6})/?").unwrap(),
        /* Full URL */ 
        Regex::new(r"^https://www\.reddit\.com/.+comments/([\w\d]{6})/?").unwrap()];

    for re in regexes.iter() {
        let captures = re.captures(input);
        if captures.is_none() { continue; }
        
        return Some(captures.unwrap().get(1).unwrap().as_str().to_owned());
    }

    None
}

fn get_valid_reddit_url(post_id: &str) -> String {
    format!("https://www.reddit.com/{}/.json", post_id)
}

fn save_post(content: String, path: std::path::PathBuf) {
    let filename = std::path::Path::new("output.json");
    let full_path = path.join(filename);
    let mut file = File::create(full_path).expect("Couldn't create file, do you have access to the provived output path?");
    file.write_all(content.as_bytes()).expect("Couldn't write file, do you have rights do modify files in the provived output path?");
}

fn get_reddit_post(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url)
        .header(USER_AGENT, "backuppit")
        .send();
    
    match response {
        Ok(resp) => Ok(resp.text()?),
        Err(e) => Err(e.into())
    }
}

fn main() {
    let args = CliArgs::parse();

    let url = match try_extract_post_id(&args.post_id) {
        Some(post_id) => get_valid_reddit_url(&post_id),
        None => panic!("Post ID is not valid")
    };

    let post_content = match get_reddit_post(&url) {
        Ok(content) => content,
        Err(e) => panic!("{}", e)
    };

    save_post(post_content, args.output);
    println!("Post saved succesfully!");
}
