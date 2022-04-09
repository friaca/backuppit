use serde_json::Value;
use chrono::{NaiveDateTime, DateTime, Utc};

// https://stackoverflow.com/a/50072164/9426143
fn timestamp_to_readable(str_timestamp: String) -> String {
  let timestamp = str_timestamp.parse::<f64>().unwrap();
  let naive = NaiveDateTime::from_timestamp(timestamp.floor() as i64, 0);
  let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

  datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

// TODO: Create Reddit link struct to avoid converting every field
fn as_str(value: &Value) -> &str {
  value.as_str().unwrap()
}

pub fn format_md_file(content: &serde_json::Value) -> String {
  format!(
"---
Author: {}
Subreddit: {}
Title: {}
Date: {}
---
{}", 
  as_str(&content["author"]),
  as_str(&content["subreddit"]), 
  as_str(&content["title"]),
  timestamp_to_readable(content["created_utc"].to_string()),
  as_str(&content["selftext"]))
}