use crate::utils::responds_build;
use serenity::builder::{CreateCommand, CreateEmbed, CreateInteractionResponseMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Instant;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    // get the time when the bot received the interaction
    let start = Instant::now();
    if let Err(_) = ctx.http.get_gateway().await {
        return Err(serenity::Error::Other("Failed to get gateway"));
    }
    let duration: std::time::Duration = start.elapsed();

    let embed: CreateEmbed = CreateEmbed::new()
        .title("Ping 🏓")
        .description(format!("📡 Latency: {}ms", duration.as_millis()))
        .color(0x00FF00);

    responds_build::send(
        ctx,
        interaction,
        CreateInteractionResponseMessage::new().embed(embed),
    )
    .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("Get latency bot to discord gateway")
}
