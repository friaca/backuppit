use clap::Parser;
use regex::Regex;
use reqwest::header::USER_AGENT;

mod response;
mod template;

use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
pub struct CliArgs {
  /// Reddit's post ID, can be a full URL or a fullname 6 characters long ID
  #[clap(long = "id")]
  post_id: String,
  /// Path to save the post
  #[clap(short, long, parse(from_os_str))]
  output: std::path::PathBuf,
}

fn get_post_info(response_json: &response::BasicListingVec<serde_json::Value>) -> serde_json::Value {
  response_json[0].data.children[0].data.clone()
}



fn extract_post_id(input: &str) -> Option<String> {
  let regexes = [
    /* Singular ID */
    Regex::new(r"^([\w\d]{6})$").unwrap(),
    /* Half URL */
    Regex::new(r"^https://www\.reddit\.com/([\w\d]{6})/?").unwrap(),
    /* Full URL */
    Regex::new(r"^https://www\.reddit\.com/.+comments/([\w\d]{6})/?").unwrap(),
  ];

  for re in regexes.iter() {
    let captures = re.captures(input);
    if captures.is_none() {
      continue;
    }

    return Some(captures.unwrap().get(1).unwrap().as_str().to_owned());
  }

  None
}

fn format_reddit_url(post_id: &str) -> String {
  format!("https://www.reddit.com/{}/.json", post_id)
}

fn save_file(content: String, path: std::path::PathBuf) {
  let filename = std::path::Path::new("output.md");
  let full_path = path.join(filename);
  let mut file = File::create(full_path)
    .expect("Couldn't create file, do you have access to the provived output path?");
  file
    .write_all(content.as_bytes())
    .expect("Couldn't write file, do you have rights do modify files in the provived output path?");
}

fn get_reddit_post(url: &str) -> Result<response::BasicListingVec<serde_json::Value>, Box<dyn Error>> {
  let client = reqwest::blocking::Client::new();
  let response = client.get(url).header(USER_AGENT, "backuppit").send()?;

  Ok(response.json()?)
}

pub fn run(args: CliArgs) {
  let url = match extract_post_id(&args.post_id) {
    Some(post_id) => format_reddit_url(&post_id),
    None => panic!("Invalid post ID"),
  };

  let post_content = match get_reddit_post(&url) {
    Ok(response) => get_post_info(&response),
    Err(e) => panic!("Invalid response: {:?}", e)
  };

  let file_content = template::format_md_file(&post_content);

  save_file(file_content, args.output);
  println!("Post saved succesfully!");
}
