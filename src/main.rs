mod commands;
mod handler;

use std::env;
use serenity::prelude::*;
use std::{sync::OnceLock, time::Instant};
use handler::Handler;

static START: OnceLock<Instant> = OnceLock::new();

const OP_TEXT: &str = r"
  _____  ______ _____ __  __  ____  _    _ _____  
 |  __ \|  ____|_   _|  \/  |/ __ \| |  | |  __ \ 
 | |  | | |__    | | | \  / | |  | | |  | | |__) |
 | |  | |  __|   | | | |\/| | |  | | |  | |  _  / 
 | |__| | |____ _| |_| |  | | |__| | |__| | | \ \ 
 |_____/|______|_____|_|  |_|\____/ \____/|_|  \_\                                       

 VERSION 0.1.0
";

#[tokio::main]
async fn main() {
    println!("{}", OP_TEXT);

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