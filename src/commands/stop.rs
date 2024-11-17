use crate::lavalink_handler::LAVALINK_CLIENT;
use crate::utils::responsebuild;
use crate::Error;
use serenity::builder::{CreateCommand, CreateEmbed, CreateInteractionResponseMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
    let lavalink = LAVALINK_CLIENT
        .get()
        .expect("LavalinkClient not initialized")
        .clone();

    let guild_id = interaction.guild_id.unwrap();

    let Some(player) = lavalink.get_player_context(guild_id) else {
        responsebuild::send_response(ctx, interaction, "Join the bot to a voice channel first")
            .await?;
        return Ok(());
    };

    let now_playing = player.get_player().await?.track;

    if let Some(np) = now_playing {
        player.stop_now().await?;
        let stop_embed: CreateEmbed = CreateEmbed::new()
            .title("Stopped")
            .color(0x00FF00)
            .description(format!("Stopped playing {}", np.info.title));

        responsebuild::send(
            CreateInteractionResponseMessage::new().embed(stop_embed),
            ctx,
            interaction,
        )
        .await?;
    } else {
        responsebuild::send_response(ctx, interaction, "Nothing is playing").await?;
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("stop").description("Stop the music")
}
