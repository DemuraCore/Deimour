use crate::utils::voice;
use serenity::all::CreateInteractionResponse;
use serenity::builder::CreateCommand;
use serenity::builder::CreateInteractionResponseMessage;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub fn register() -> CreateCommand {
    CreateCommand::new("join").description("Join a voice channel")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let guild_id = interaction.guild_id.unwrap();

    // Check if the bot is already in a voice channel
    if let Some(handler_lock) = songbird::get(ctx).await.unwrap().get(guild_id) {
        let handler = handler_lock.lock().await;
        if handler.current_channel().is_some() {
            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("I'm already in a voice channel"),
            );
            interaction.create_response(ctx, response).await?;
            return Ok(());
        }
    }

    // Call the join_voice_channel function
    let has_joined: bool = voice::join_voice_channel(ctx, interaction).await?;

    // If the bot has successfully joined a voice channel
    if has_joined {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("Joined voice channel"),
        );
        interaction.create_response(ctx, response).await?;
    } else {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("Failed to join voice channel"),
        );
        interaction.create_response(ctx, response).await?;
    }

    Ok(())
}
