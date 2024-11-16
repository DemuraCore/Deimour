use serenity::all::CreateInteractionResponse;
use serenity::builder::{CreateCommand, CreateEmbed, CreateInteractionResponseMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Instant;

pub async  fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    // get the time when the bot received the interaction
    let start = Instant::now();
    if let Err(_) = ctx.http.get_gateway().await {
        return Err(serenity::Error::Other("Failed to get gateway"));
    }
    let duration = start.elapsed();


    let embed = CreateEmbed::new()
        .title("Ping")
        .description(format!("Latency: {}ms", duration.as_millis()))
        .color(0x00FF00);

    // create embed response
    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embed(embed)
    );

    // send response
    interaction.create_response(ctx, response).await?;

    

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("Get latency bot to discord gateway")
}