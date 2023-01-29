mod commands;
mod handler;

use crate::handler::Handler;
use dotenv::dotenv;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = dotenv::var("DISCORD_TOKEN").unwrap();

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
