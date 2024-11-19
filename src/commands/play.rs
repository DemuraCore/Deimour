use crate::lavalink_handler::LAVALINK_CLIENT;
use crate::utils::format_duration;
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
use std::time::Instant;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
    let start = Instant::now();

    let song = interaction.data.options.get(0);
    let source = interaction.data.options.get(1);
    let lavalink = LAVALINK_CLIENT
        .get()
        .expect("LavalinkClient not initialized")
        .clone();

    let guild_id = interaction.guild_id.unwrap();
    let has_joined = voice::join_voice_channel(ctx, interaction).await?;

    if !has_joined {
        return Ok(());
    }

    let player = match lavalink.get_player_context(GuildId::from(guild_id)) {
        Some(player) => player,
        None => {
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
        }
    };

    let query = song.and_then(|x| {
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
            match source {
                Some(source) => match source.value {
                    CommandDataOptionValue::String(ref s) => match s.as_str() {
                        "youtube" => SearchEngines::YouTube.to_query(&query)?,
                        "soundcloud" => SearchEngines::SoundCloud.to_query(&query)?,
                        "dezeer" => SearchEngines::Deezer.to_query(&query)?,
                        "spotify" => SearchEngines::Spotify.to_query(&query)?,
                        _ => SearchEngines::SoundCloud.to_query(&query)?,
                    },
                    _ => SearchEngines::SoundCloud.to_query(&query)?,
                },
                None => SearchEngines::SoundCloud.to_query(&query)?,
            }
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
        let duration = start.elapsed();

        let queue_embed = CreateEmbed::new()
            .title("Added to queue")
            .color(0x00FF00)
            .description(format!(
                "[{} - {}](<{}>)",
                track.info.author,
                track.info.title,
                track.info.uri.as_deref().unwrap_or("")
            ))
            .footer(CreateEmbedFooter::new(format!(
                "Deimour took {}ms to respond",
                duration.as_millis()
            )))
            .fields(vec![
                ("Requested by", interaction.user.name.clone(), true),
                ("Duration", format_duration(track.info.length / 10000), true),
            ]);

        responds_build::send(
            ctx,
            interaction,
            CreateInteractionResponseMessage::new().embed(queue_embed),
        )
        .await?;
    }

    for track in &mut tracks {
        track.track.user_data = Some(
            serde_json::json!({"requester_username": interaction.user.name, "requester_id": interaction.user.id}),
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
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "source",
                "The source of the song",
            )
            .add_string_choice("Youtube", "youtube")
            .add_string_choice("Soundcloud", "soundcloud")
            .add_string_choice("Spotify", "spotify")
            .add_string_choice("dezeer", "dezeer"),
        )
        .description("Play a song")
}
