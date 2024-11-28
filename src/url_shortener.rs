use std::collections::HashMap;
use std::fs::File;
use std::io;
use serde_json;
//use rand::random;
use url::Url;

pub fn run_url_shortener() {
    let file_name = "urls.json";
    let mut urls = load_urls_from_file(file_name); // Load existing mappings

    let mut input = String::new();
    println!("Enter the URL to shorten:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let long_url = input.trim();

    // Validate the URL
    if long_url.is_empty() {
        println!("Error: URL cannot be empty.");
        return;
    }
    if Url::parse(long_url).is_err() {
        println!("Error: Invalid URL format.");
        return;
    }

    let short_url = shorten_url(long_url, &mut urls); // Generate short URL
    save_urls_to_file(&urls, file_name); // Save updated mappings

    println!("Long URL: {}", long_url);
    println!("Short URL: {}", short_url);
}

fn shorten_url(url: &str, urls: &mut HashMap<String, String>) -> String {
    let mut short = format!("short.ly/{}", url.hash());
    // Check for collision
    while urls.contains_key(&short) {
        // Append a random suffix to make it unique
        let random_suffix: u32 = rand::random();
        short = format!("short.ly/{}{}", url.hash(), random_suffix);
    }
    urls.insert(short.clone(), url.to_string());
    short
}

trait Hash {
    fn hash(&self) -> String;
}

impl Hash for &str {
    fn hash(&self) -> String {
        let hash = md5::compute(self);
        format!("{:x}", hash)
    }
}

// Save the URL mappings to a file
fn save_urls_to_file(urls: &HashMap<String, String>, file_name: &str) {
    let file = File::create(file_name).expect("Unable to create file");
    serde_json::to_writer(file, urls).expect("Failed to write to file");
}

// Load URL mappings from a file
fn load_urls_from_file(file_name: &str) -> HashMap<String, String> {
    let file = File::open(file_name).unwrap_or_else(|_| File::create(file_name).expect("Failed to create file"));
    serde_json::from_reader(file).unwrap_or_else(|_| HashMap::new())
}
