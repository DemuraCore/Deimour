use crate::lavalink_handler::LAVALINK_CLIENT;
use crate::utils::responds_build;
use crate::utils::voice;
use crate::Error;
use lavalink_rs::model::GuildId;
use lavalink_rs::prelude::*;
use serenity::all::CreateInteractionResponse;
use serenity::builder::{
    CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponseMessage,
};
use serenity::model::application::CommandOptionType;
use serenity::model::prelude::*;
use serenity::prelude::*;

// start timer for how long the bot takes to respond
use std::time::Instant;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
    // start timer for how long the bot takes to respond
    let start = Instant::now();

    let song = interaction.data.options.get(0);
    let lavalink = LAVALINK_CLIENT
        .get()
        .expect("LavalinkClient not initialized")
        .clone();

    let guild_id = interaction.guild_id.unwrap();
    let has_joined: bool = voice::join_voice_channel(ctx, interaction).await?;

    let Some(player) = lavalink.get_player_context(GuildId::from(interaction.guild_id.unwrap()))
    else {
        interaction
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Join the bot to a voice channel first."),
                ),
            )
            .await?;
        return Ok(());
    };

    if !has_joined {
        return Ok(());
    }

    let query: Option<String> = song.and_then(|x| {
        if let CommandDataOptionValue::String(ref s) = x.value {
            Some(s.clone())
        } else {
            None
        }
    });

    let search = if let Some(query) = query {
        if query.starts_with("http") {
            query
        } else {
            SearchEngines::YouTube.to_query(&query)?
        }
    } else {
        if let Ok(player_data) = player.get_player().await {
            let queue = player.get_queue();

            if player_data.track.is_none() && queue.get_track(0).await.is_ok_and(|x| x.is_some()) {
                player.skip()?;
            } else {
                interaction
                    .create_response(
                        ctx,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new().content("Nothing is playing"),
                        ),
                    )
                    .await?;
            }
        }

        return Ok(());
    };

    let loaded_track = lavalink.load_tracks(guild_id, &search).await?;

    let mut playlist_info: Option<String> = None;

    let mut tracks: Vec<TrackInQueue> = match loaded_track.data {
        Some(TrackLoadData::Track(x)) => vec![x.into()],
        Some(TrackLoadData::Search(x)) => vec![x[0].clone().into()],
        Some(TrackLoadData::Playlist(x)) => {
            playlist_info = Some(x.info.name);
            x.tracks.iter().map(|x| x.clone().into()).collect()
        }

        _ => {
            interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(format!("{:?}", loaded_track)),
                    ),
                )
                .await?;
            return Ok(());
        }
    };

    if let Some(info) = playlist_info {
        // ctx.say(format!("Added playlist to queue: {}", info.name,))
        //     .await?;
        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("Added playlist to queue: {}", info)),
                ),
            )
            .await?;
    } else {
        let track = &tracks[0].track;

        if let Some(uri) = &track.info.uri {
            let duration: std::time::Duration = start.elapsed();

            let queue_embed: CreateEmbed = CreateEmbed::new()
                .title("Added to queue")
                .color(0x00FF00)
                .description(format!(
                    "[{} - {}](<{}>)",
                    track.info.author, track.info.title, uri
                ))
                .footer(CreateEmbedFooter::new(format!(
                    "Deimour took {}ms to respond",
                    duration.as_millis()
                )))
                .fields(
                    vec![
                        ("Requested by", interaction.user.name.clone(), true),
                        (
                            "Duration",
                            format!("{} seconds", track.info.length / 1000),
                            true,
                        ),
                    ]
                    .into_iter()
                    .map(|(name, value, inline)| (name, value, inline)),
                );

            responds_build::send(
                ctx,
                interaction,
                CreateInteractionResponseMessage::new().embed(queue_embed),
            )
            .await?;
        } else {
            let duration: std::time::Duration = start.elapsed();
            let queue_embed: CreateEmbed = CreateEmbed::new()
                .title("Added to queue")
                .color(0x00FF00)
                .description(format!("{} - {}", track.info.author, track.info.title))
                .fields(
                    vec![
                        ("Requested by", interaction.user.name.clone(), true),
                        (
                            "Duration",
                            format!("{} seconds", track.info.length / 1000),
                            true,
                        ),
                    ]
                    .into_iter()
                    .map(|(name, value, inline)| (name, value, inline)),
                )
                .footer(CreateEmbedFooter::new(format!(
                    "Deimour took {}ms to respond",
                    duration.as_millis()
                )));

            responds_build::send(
                ctx,
                interaction,
                CreateInteractionResponseMessage::new().embed(queue_embed),
            )
            .await?;
        }
    }
    for i in &mut tracks {
        i.track.user_data = Some(
            serde_json::json!({"requester_username": interaction.user.name,
            "requester_id": interaction.user.id}),
        );
    }

    let queue = player.get_queue();
    queue.append(tracks.into())?;

    if let Ok(player_data) = player.get_player().await {
        if player_data.track.is_none() && queue.get_track(0).await.is_ok_and(|x| x.is_some()) {
            player.skip()?;
        }
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("play")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "song", "The song to play")
                .required(true),
        )
        .description("Play a song")
}
