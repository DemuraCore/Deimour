use crate::lavalink_handler::LAVALINK_CLIENT;
use crate::utils::responds_build;
use crate::Error;
use serenity::builder::{CreateCommand, CreateEmbed, CreateInteractionResponseMessage};
use serenity::model::{colour, prelude::*};
use serenity::prelude::*;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
    let lavalink = LAVALINK_CLIENT
        .get()
        .expect("LavalinkClient not initialized")
        .clone();

    let guild_id = interaction.guild_id.unwrap();

    let Some(player) = lavalink.get_player_context(guild_id) else {
        responds_build::send_response(ctx, interaction, "Join the bot to a voice channel first")
            .await?;
        return Ok(());
    };

    let now_playing = player.get_player().await?.track;
    // replay the current song
    if let Some(np) = now_playing {
        player
            .set_position(std::time::Duration::from_secs(0))
            .await?;
        let replay_embed: CreateEmbed = CreateEmbed::new()
            .title("Replayed")
            .color(colour::Colour::DARK_GREEN)
            .description(format!("Replayed {}", np.info.title));

        responds_build::send(
            ctx,
            interaction,
            CreateInteractionResponseMessage::new().embed(replay_embed),
        )
        .await?;
    } else {
        responds_build::send_response(ctx, interaction, "Nothing is playing").await?;
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("replay").description("Replay the current song")
}
