use crate::Error;
use serenity::all::CreateInteractionResponse;
use serenity::all::{CreateEmbed, CreateInteractionResponseMessage};
use serenity::model::prelude::*;
use serenity::{builder, prelude::*};

pub fn _success_embed(title: &str, description: &str) -> CreateInteractionResponse {
    let embed = CreateEmbed::new()
        .title(title)
        .description(description)
        .color(0x00FF00);

    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().add_embed(embed))
}

pub fn _error_embed(title: &str, description: &str) -> CreateInteractionResponse {
    let embed = CreateEmbed::new()
        .title(title)
        .description(description)
        .color(0xFF0000);

    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().add_embed(embed))
}

pub async fn send_response(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: impl Into<String>,
) -> Result<(), Error> {
    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(content),
            ),
        )
        .await
        .map_err(Error::from)
}

pub async fn send(
    builder: builder::CreateInteractionResponseMessage,
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(), Error> {
    interaction
        .create_response(ctx, CreateInteractionResponse::Message(builder))
        .await
        .map_err(Error::from)
}
