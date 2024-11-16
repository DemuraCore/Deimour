use serenity::all::CreateInteractionResponse;
use serenity::builder::{CreateCommand, CreateInteractionResponseMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

pub fn register() -> CreateCommand {
    CreateCommand::new("leave").description("Leave a voice channel")
}

pub async fn run(ctx: &Context,  interaction: &CommandInteraction) ->  Result<(), serenity::Error>  {
    let guild_id: GuildId = interaction.guild_id.unwrap();
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(format!("Failed to leave voice channel: {:?}", e))
            );
            return interaction.create_response(ctx, response).await;
        } 

        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("Left voice channel")
        );

        return interaction.create_response(ctx, response).await;
    } else {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("I'm not in a voice channel")
        );
        interaction.create_response(ctx, response).await?;
    }


    

    Ok(())
}


