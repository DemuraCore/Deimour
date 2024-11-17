use colored::Colorize;
use lavalink_rs::{hook, model::events, prelude::*};
use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::futures;
use serenity::http::Http;
use serenity::model::id::ChannelId;

#[hook]
pub async fn raw_event(_: LavalinkClient, session_id: String, event: &serde_json::Value) {
    if event["op"].as_str() == Some("event") || event["op"].as_str() == Some("playerUpdate") {
        println!(
            "{} Lavalink raw event: {:?}",
            "[Lavalink]".green().bold(),
            event
        );
    }
}

#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, _event: &events::Ready) {
    client.delete_all_player_contexts().await.unwrap();
    println!(
        "{} Lavalink ready: {:?}",
        "[Lavalink]".green().bold(),
        session_id
    );
}

#[hook]
pub async fn track_start(client: LavalinkClient, _session_id: String, event: &events::TrackStart) {
    let player_context = client.get_player_context(event.guild_id).unwrap();
    let data = player_context
        .data::<(ChannelId, std::sync::Arc<Http>)>()
        .unwrap();
    let (channel_id, http) = (&data.0, &data.1);

    let embed_track = {
        let track = &event.track;

        if let Some(uri) = &track.info.uri {
            println!(
                "{} Now playing: {} - {}",
                "[Lavalink]".green().bold(),
                track.info.author,
                track.info.title
            );
            CreateEmbed::new()
                .title("Now playing")
                .color(0x00FF00)
                .description(format!(
                    "Now playing: [{} - {}](<{}>) ",
                    track.info.author, track.info.title, uri,
                ))
                .footer(CreateEmbedFooter::new(format!(
                    "Requested by {}",
                    track.user_data.clone().unwrap()["requester_username"]
                        .as_str()
                        .unwrap_or_default()
                        .replace("\"", "")
                )))
        } else {
            CreateEmbed::new()
                .title("Now playing")
                .color(0x00FF00)
                .description(format!(
                    "Now playing: {} - {}",
                    track.info.author, track.info.title
                ))
                .footer(CreateEmbedFooter::new(format!(
                    "Requested by {}",
                    track.user_data.clone().unwrap()["requester_username"]
                        .as_str()
                        .unwrap_or_default()
                        .replace("\"", "")
                )))
        }
    };

    let _ = channel_id
        .send_message(http, CreateMessage::new().add_embed(embed_track))
        .await;
}

#[hook]
pub async fn track_end(client: LavalinkClient, _session_id: String, event: &events::TrackEnd) {
    let player_context = client.get_player_context(event.guild_id).unwrap();
    let data = player_context
        .data::<(ChannelId, std::sync::Arc<Http>)>()
        .unwrap();
    let (channel_id, http) = (&data.0, &data.1);

    let embed = {
        let track = &event.track;

        if let Some(uri) = &track.info.uri {
            CreateEmbed::new()
                .title("Finished playing")
                .description(format!(
                    "Finished playing: [{} - {}](<{}>)",
                    track.info.author, track.info.title, uri
                ))
                .footer(CreateEmbedFooter::new(format!(
                    "Requested by {}",
                    track.user_data.clone().unwrap()["requester_username"]
                        .as_str()
                        .unwrap_or_default()
                        .replace("\"", "")
                )))
        } else {
            CreateEmbed::new()
                .title("Finished playing")
                .description(format!(
                    "Finished playing: {} - {}",
                    track.info.author, track.info.title
                ))
                .footer(CreateEmbedFooter::new(format!(
                    "Requested by {}",
                    track.user_data.clone().unwrap()["requester_username"]
                        .as_str()
                        .unwrap_or_default()
                        .replace("\"", "")
                )))
        }
    };

    let _ = channel_id
        .send_message(http, CreateMessage::new().add_embed(embed))
        .await;
}
