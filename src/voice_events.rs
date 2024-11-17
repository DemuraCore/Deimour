use lavalink_rs::{hook, model::events, prelude::*};
use serenity::futures;
use serenity::http::Http;
use serenity::model::id::ChannelId;

#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, event: &events::Ready) {
    client.delete_all_player_contexts().await.unwrap();
    println!("Lavalink ready: {:?}", session_id);
}

#[hook]
pub async fn track_start(client: LavalinkClient, _session_id: String, event: &events::TrackStart) {
    let player_context = client.get_player_context(event.guild_id).unwrap();
    let data = player_context
        .data::<(ChannelId, std::sync::Arc<Http>)>()
        .unwrap();
    let (channel_id, http) = (&data.0, &data.1);

    let msg = {
        let track = &event.track;

        if let Some(uri) = &track.info.uri {
            format!(
                "Now playing: [{} - {}](<{}>) | Requested by <@{}>",
                track.info.author,
                track.info.title,
                uri,
                track.user_data.clone().unwrap()["requester_id"]
                    .as_str()
                    .unwrap_or_default()
                    .replace("\"", "")
            )
        } else {
            format!(
                "Now playing: {} - {} | Requested by <@{}>",
                track.info.author,
                track.info.title,
                track.user_data.clone().unwrap()["requester_id"]
                    .as_str()
                    .unwrap_or_default()
                    .replace("\"", "")
            )
        }
    };

    let _ = channel_id.say(http, msg).await;
}

#[hook]
pub async fn track_end(client: LavalinkClient, _session_id: String, event: &events::TrackEnd) {
    let player_context = client.get_player_context(event.guild_id).unwrap();
    let data = player_context
        .data::<(ChannelId, std::sync::Arc<Http>)>()
        .unwrap();
    let (channel_id, http) = (&data.0, &data.1);

    let msg = {
        let track = &event.track;

        if let Some(uri) = &track.info.uri {
            format!(
                "Finished playing: [{} - {}](<{}>) | Requested by <@{}>",
                track.info.author,
                track.info.title,
                uri,
                track.user_data.clone().unwrap()["requester_id"]
                    .as_str()
                    .unwrap_or_default()
                    .replace("\"", "")
            )
        } else {
            format!(
                "Finished playing: {} - {} | Requested by <@{}>",
                track.info.author,
                track.info.title,
                track.user_data.clone().unwrap()["requester_id"]
                    .as_str()
                    .unwrap_or_default()
                    .replace("\"", "")
            )
        }
    };

    let _ = channel_id.say(http, msg).await;
}
