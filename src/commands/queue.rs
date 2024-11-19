use crate::lavalink_handler::LAVALINK_CLIENT;
use crate::utils::responds_build;
use crate::Error;
use serenity::builder::{CreateCommand, CreateEmbed, CreateInteractionResponseMessage};
use serenity::futures::{future, StreamExt};
use serenity::model::{colour, prelude::*};
use serenity::prelude::*;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
    let lavalink = LAVALINK_CLIENT
        .get()
        .expect("LavalinkClient not initialized")
        .clone();

    let guild_id = interaction.guild_id.unwrap();
    let Some(player) = lavalink.get_player_context(guild_id) else {
        responds_build::send_response(&ctx, interaction, "Join the bot to a voice channel first")
            .await?;
        return Ok(());
    };

    let queue = player.get_queue();
    let player_data = player.get_player().await?;
    let max = queue.get_count().await?.min(9);

    if let Some(track) = player_data.track {
        let time_s = player_data.state.position / 1000 % 60;
        let time_m = player_data.state.position / 1000 / 60;
        let time = format!("{:02}:{:02}", time_m, time_s);

        let emned_queue = CreateEmbed::new()
            .title("Queue")
            .color(colour::Colour::DARK_GREEN)
            .description(format!(
                "Now Playing: [{} - {}](<{}>) | {}",
                track.info.author,
                track.info.title,
                track.info.uri.unwrap(),
                time
            ))
            .field(
                "List",
                queue
                    .enumerate()
                    .take_while(|(idx, _)| future::ready(*idx < max))
                    .map(|(idx, x)| {
                        if let Some(uri) = &x.track.info.uri {
                            format!(
                                "{} -> [{} - {}](<{}>) ",
                                idx + 1,
                                x.track.info.author,
                                x.track.info.title,
                                uri,
                            )
                        } else {
                            format!(
                                "{} -> {} - {} ",
                                idx + 1,
                                x.track.info.author,
                                x.track.info.title,
                            )
                        }
                    })
                    .collect::<Vec<_>>()
                    .await
                    .join("\n"),
                false,
            );
        responds_build::send(
            ctx,
            interaction,
            CreateInteractionResponseMessage::new().embed(emned_queue),
        )
        .await?;
    } else {
        let emned_queue = CreateEmbed::new()
            .title("Queue")
            .color(colour::Colour::DARK_GREEN)
            .description("Nothing is playing");
        responds_build::send(
            ctx,
            interaction,
            CreateInteractionResponseMessage::new().embed(emned_queue),
        )
        .await?;
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("queue").description("Show the queue")
}
