mod commands;
mod handler;
mod utils;

use crate::handler::Handler;
use dotenv::dotenv;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = dotenv::var("DISCORD_TOKEN").unwrap();

    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    let handler = Handler { database };

    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
