use std::fs;
use yt_dlp::model::Video;

fn main() {
    let content = fs::read_to_string("example.json").expect("Failed to read example.json");
    match serde_json::from_str::<Video>(&content) {
        Ok(_) => println!("Successfully parsed example.json"),
        Err(e) => println!("Failed to parse example.json: {}", e),
    }
}
