use clap::Parser; // command-line argument parser crate
use reqwest::Client; // HTTP client that makes requests to web servers crate
use serde_json::Value; // Parse JSON string to Rust types and vice versa crate
use std::env; // Rust standard environment crate
use dotenv::dotenv; // Crate for loading environment variable from .env file

#[derive(Parser)] // macro to generates necessary code for parse command-line arguments based on struct definition

// Attribute for providing metadata about the command-line interface
#[command(author = "Quotes4Days", version = "1.0", about = "A CLI Quote Generator", long_about = None)]

// Struct to define command-line arguments
struct Cli {
    #[arg(short, long, help = "Number of tweet to process")]
    count: Option<u8>,
    //   attribute describes each argument. In this case:
    //   short: the short version of the argument
    //   long: the long version of the argument
    //   help: the help message to display when the user requests help
    //   Option<u8>: the type of the argument, in this case, an unsigned 8-bit integer
    //   count is an optional argument specifying the number of tweets to process.
}

// Asynchronous Function to get my twitter ID using twitter API

async fn get_user_id(client: &Client, twitter_handle: &str, access_token: &str) -> Option<String> {
    let url = format!("https://api.twitter.com/2/users/by/username/{}", twitter_handle);
    let response = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await
        .expect("Failed to send request");


    // Handle the response
    if response.status().is_success() {
        let user_info: Value = response.json().await.expect("Failed to parse JSON");
        if let Some(twitter_id) = user_info["data"]["id"].as_str() {
            return Some(twitter_id.to_string());
        }
    } else {
        let status = response.status();
        let response_text = response.text().await.expect("Failed to read the response text");
        eprintln!("Error fetching twitter ID: {:?}, Body: {}", status, response_text);
    }
    None
}

// Asynchronous Function to fetch my tweets

async fn fetch_user_tweets(client: &Client, twitter_id: &str, count: u8, access_token: &str) {
    let url = format!("https://api.twitter.com/2/users/{}/tweets?max_results={}", twitter_id, count);
    let response = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await
        .expect("Failed to send request");

    // Handle the response
    if response.status().is_success() {
        let tweets: Value = response.json().await.expect("Failed to parse JSON");
        if let Some(tweets_arr) = tweets["data"].as_array() {
            for (i, tweet) in tweets_arr.iter().enumerate() {
                if let Some(tweet_content) = tweet["text"].as_str() {
                    println!("Quote {}:\n {}\n", i + 1, tweet_content);
                }
            }
        } else {
            println!("No tweets found for this user.");
        }
    } else {
        let status = response.status();
        let response_text = response.text().await.expect("Failed to read the response text");
        eprintln!("Error fetching tweets: {:?}, Body: {}", status, response_text);
    }
}


#[tokio::main] // Defines entry point of an asynchronous application
async fn main() {
    // Load environment variables from the .env file
    dotenv().ok();
    let cli = Cli::parse();

    // Fetch Twitter API keys from environment variables
    let access_token = env::var("TWITTER_BEARER_TOKEN").expect("Missing TWITTER_BEARER_TOKEN");
    let client = Client::new();

    let count = cli.count.unwrap_or(5); // Default to 5 tweets if count is not specified
    let twitter_handle = "QuoteGuy08"; // My Twitter handle, follow me for more quotes, haha

    // Fetching my twitter ID
    if let Some(twitter_id) = get_user_id(&client, twitter_handle, &access_token).await {
        // Fetching my tweets
        fetch_user_tweets(&client, &twitter_id, count, &access_token).await;
    } else {
        eprintln!("Failed to retrieve user twitter ID.");
    }
}