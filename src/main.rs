mod commands;
mod handler;

use std::env;
use serenity::prelude::*;
use std::{sync::OnceLock, time::Instant};
use handler::Handler;

static START: OnceLock<Instant> = OnceLock::new();

#[tokio::main]
async fn main() {
    START.get_or_init(|| Instant::now());
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}