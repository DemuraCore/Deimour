mod commands;
mod handler;
mod lavalink_handler;
mod utils;
mod voice_events;

use colored::Colorize;
use handler::Handler;
use lavalink_handler::initialize_lavalink_client;
use serenity::prelude::*;
use songbird::SerenityInit;
use std::env;
use std::{sync::OnceLock, time::Instant};

static START: OnceLock<Instant> = OnceLock::new();

pub type Error = Box<dyn std::error::Error + Send + Sync>;

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
    if let Err(_) = dotenv::dotenv() {
        println!("Note: .env file not found, using default environment variables");
    }

    let token: String = env::var("DISCORD_TOKEN").expect(
        format!(
            "{} Expected DISCORD_TOKEN in environment",
            "[READY]".red().bold()
        )
        .as_str(),
    );

    let mut client: Client = Client::builder(token, GatewayIntents::all())
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Error creating client");
    // get user id from client

    initialize_lavalink_client().await;

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
