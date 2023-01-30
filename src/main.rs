use std::env;

use http::{Request, Response};
use roux::Reddit;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let clientId: String = std::env::var("REDDIT_CLIENT_ID").expect("There must be a reddit client id.");
    let clientSecret: String = std::env::var("REDDIT_CLIENT_SECRET").expect("There must be a reddit client secret.")

    let accountUsername: String = std::env::var("REDDIT_USERNAME").expect("There must be a username");
    let accountUsername: String = std::env::var("REDDIT_PASSWORD").expect("REDDIT_PASSWORD");


    let client: Reddit = Reddit::new("windows11:redditScraperTool:v0.0.1 (by /u/Zie84)", &clientId, &clientSecret);

    client.username("Zie84").password()
    
}