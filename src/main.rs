use dotenv::dotenv;
use roux::{me::response::MeData, util::RouxError, Me, Reddit};
use std::io::{stdout, Write};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client_id: String =
        std::env::var("REDDIT_CLIENT_ID").expect("There must be a reddit client id.");
    let client_secret: String =
        std::env::var("REDDIT_CLIENT_SECRET").expect("There must be a reddit client secret.");

    let username: String = std::env::var("REDDIT_USERNAME").expect("There must be a username");
    let password: String =
        std::env::var("REDDIT_PASSWORD").expect("There must be a configured password");

    let me = login_to_reddit(&client_id, &client_secret, &username, &password).await;

    if me.is_ok() {
        let me_data_call = me.unwrap().me().await;
        let me_data = me_data_call.unwrap();
        println!("{}", me_data.comment_karma);
    } else {
        let error = me.unwrap_err();    
        println!("{}", error.to_string());
    }
}

async fn login_to_reddit(
    client_id: &str,
    client_secret: &str,
    username: &str,
    password: &str,
) -> Result<Me, RouxError> {
    let client = Reddit::new(
        "windows:FormattedPostDisassembler:v0.1.1 (by /u/Embarrassed_Salad918)",
        client_id,
        client_secret,  
    )
    .username(username)
    .password(password)
    .login()
    .await;

    return client;
}
