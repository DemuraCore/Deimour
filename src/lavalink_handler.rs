use crate::voice_events;
use colored::Colorize;
use lavalink_rs::client::LavalinkClient;
use lavalink_rs::{model::events, prelude::*};
use std::env;
use std::sync::Arc;
use tokio::sync::OnceCell;

pub static LAVALINK_CLIENT: OnceCell<Arc<LavalinkClient>> = OnceCell::const_new();

pub async fn initialize_lavalink_client() {
    println!(
        "{} Initializing Lavalink client...",
        "[Lavalink]".green().bold()
    );
    let events = events::Events {
        raw: Some(voice_events::raw_event),
        track_end: Some(voice_events::track_end),
        ready: Some(voice_events::ready_event),
        track_start: Some(voice_events::track_start),

        ..Default::default()
    };

    let node = NodeBuilder {
        hostname: env::var("LAVALINK_HOST").expect("Expected LAVALINK_HOST in environment"),
        is_ssl: false,
        events: events::Events::default(),
        password: env::var("LAVALINK_PASSWORD").expect("Expected LAVALINK_PASSWORD in environment"),
        user_id: UserId(1307263482789367879),
        session_id: None,
    };

    let client =
        LavalinkClient::new(events, vec![node], NodeDistributionStrategy::round_robin()).await;

    let _ = LAVALINK_CLIENT.set(Arc::new(client));
}
