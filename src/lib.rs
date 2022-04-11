use clap::Parser;
use regex::Regex;
use serde_json::Value;
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

fn get_post_info(
  response_json: &response::BasicListingVec<serde_json::Value>,
) -> serde_json::Value {
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

  for re in regexes {
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

fn format_reddit_media_url(media_id: &str, extension: &str) -> String {
  format!("https://i.redd.it/{}.{}", media_id, extension)
}

fn get_media_filename_from_url(url: &str) -> String {
  let filename_regex = Regex::new(r"/([\w\d]{13}\.\w+)$").unwrap();
  filename_regex.captures(url).unwrap().get(1).unwrap().as_str().to_owned()
}

fn try_save_images(post_content: &Value, path: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
  if post_content["is_reddit_media_domain"].as_bool().unwrap() {
    save_image(
      &post_content["url"].as_str().unwrap(), 
      &get_media_filename_from_url(&post_content["url"].as_str().unwrap()), 
      path
    )?
  } else if post_content["is_gallery"].as_bool().unwrap() {
    let image_urls = get_gallery_urls(&post_content["media_metadata"]);

    for url in image_urls {
      save_image(&url, &get_media_filename_from_url(&url), path)?;
    }
  }

  println!("Image(s) saved successfully!");
  Ok(())
}

fn get_gallery_urls(media_metadata: &Value) -> Vec<String> {
  let mut urls: Vec<String> = vec![];

  for media in media_metadata.as_object() {
    for (_, value) in media.iter() {
      // value["m"] == "image/png" | "image/jpg" | ...
      let media_type = &value["m"].as_str().unwrap()[..6];
      if media_type != "image/" { continue; }

      let extension = &value["m"].as_str().unwrap()[6..];
      urls.push(format_reddit_media_url(value["id"].as_str().unwrap(), &extension));
    }
  }

  urls
}

fn save_image(url: &str, filename: &str, path: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
  let filename = std::path::Path::new(filename);
  let client = reqwest::blocking::Client::new();
  let mut image_response = client
    .get(url)
    .header(USER_AGENT, "backuppit v0.1.0")
    .send()?;

  let mut file = File::create(&path.join(filename)).expect("Couldn't create image file: ");
  std::io::copy(&mut image_response, &mut file).expect("Couldn't save image: ");

  Ok(())
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

fn get_reddit_post(
  url: &str,
) -> Result<response::BasicListingVec<serde_json::Value>, Box<dyn Error>> {
  let client = reqwest::blocking::Client::new();
  let response = client
    .get(url)
    .header(USER_AGENT, "backuppit v0.1.0")
    .send()?;

  Ok(response.json()?)
}

pub fn run(args: CliArgs) {
  let url = match extract_post_id(&args.post_id) {
    Some(post_id) => format_reddit_url(&post_id),
    None => panic!("Invalid post ID"),
  };

  let post_content = match get_reddit_post(&url) {
    Ok(response) => get_post_info(&response),
    Err(e) => panic!("Invalid response: {:?}", e),
  };

  try_save_images(&post_content, &args.output).expect("");

  let file_content = template::format_md_file(&post_content);

  save_file(file_content, args.output);
  println!("Post saved succesfully!");
}
