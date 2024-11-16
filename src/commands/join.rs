use serenity::all::CreateInteractionResponse;
use serenity::builder::{CreateCommand, CreateInteractionResponseMessage};

use songbird::events::TrackEvent;
use serenity::model::prelude::*;
use crate::handler::TrackErrorNotifier;
use serenity::prelude::*;

pub fn register() -> CreateCommand {
    CreateCommand::new("join").description("Join a voice channel")
}

pub async fn run(ctx: &Context,  interaction: &CommandInteraction) ->  Result<(), serenity::Error>  {
    let guild_id: GuildId = interaction.guild_id.unwrap();
    let guild: Guild = guild_id.to_guild_cached(&ctx.cache).unwrap().clone();
    

    let member: Member = guild.member(&ctx.http, interaction.user.id).await.unwrap().into_owned();
    let channel_id: Option<ChannelId> = guild.voice_states.get(&member.user.id).and_then(|vs| vs.channel_id);

    // check if the user is in a voice channel
    let connect_to = if let Some(channel) = channel_id {
        channel
    } else {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("You need to be in a voice channel to use this command")
        );

        interaction.create_response(ctx, response).await?;

        return Ok(());
    };

    // check if the bot is already in a voice channel
    if let Some(handler_lock) = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .get(guild_id)
    {
        let handler = handler_lock.lock().await;
        if handler.current_channel().is_some() {
            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("I'm already in a voice channel")
            );
            return interaction.create_response(ctx, response).await;
        }
    }

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        // Attach an event handler to see notifications of all track errors.
        let mut handler = handler_lock.lock().await;
        
         handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    }

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(format!("Joining channel: {}", connect_to.mention()))

    );

    interaction.create_response(ctx, response).await?;


    

    Ok(())
}


