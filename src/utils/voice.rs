use crate::lavalink_handler::LAVALINK_CLIENT;
use serenity::all::CreateInteractionResponse;
use serenity::builder::CreateInteractionResponseMessage;
use serenity::http::Http;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub async fn join_voice_channel(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<bool, SerenityError> {
    let lava_client = LAVALINK_CLIENT
        .get()
        .expect("LavalinkClient not initialized")
        .clone();

    let manager = songbird::get(ctx).await.unwrap().clone();
    let guild_id = interaction.guild_id.unwrap();
    let guild = guild_id.to_guild_cached(&ctx.cache).unwrap().clone();
    let member = guild.member(&ctx.http, interaction.user.id).await.unwrap();
    let channel_id = guild
        .voice_states
        .get(&member.user.id)
        .and_then(|vs| vs.channel_id);

    let channel_context_id = interaction.channel_id;
    let channel_context = guild.channels.get(&channel_context_id).unwrap().clone();

    if lava_client.get_player_context(guild_id).is_none() {
        let connect_to = match channel_id {
            Some(channel) => channel,
            None => {
                let user_channel_id = guild
                    .voice_states
                    .get(&interaction.user.id)
                    .and_then(|voice_state| voice_state.channel_id);
                match user_channel_id {
                    Some(channel) => channel,
                    None => {
                        let response = CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("You are not in a voice channel."),
                        );
                        interaction.create_response(ctx, response).await?;
                        return Ok(false);
                    }
                }
            }
        };

        let handler = manager.join_gateway(guild_id, connect_to).await;
        match handler {
            Ok((connection_info, _)) => {
                if let Err(why) = lava_client
                    .create_player_context_with_data::<(ChannelId, std::sync::Arc<Http>)>(
                        guild_id,
                        connection_info,
                        std::sync::Arc::new((connect_to, ctx.http.clone())),
                    )
                    .await
                {
                    let response = CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(format!("Failed to create player context: {}", why)),
                    );
                    interaction.create_response(ctx, response).await?;
                    return Ok(false);
                }
                // connect_to
                //     .say(
                //         ctx,
                //         format!("Joined voice channel: {}", connect_to.mention()),
                //     )
                //     .await?;
                channel_context
                    .say(
                        ctx,
                        format!("Joined voice channel: {}", connect_to.mention()),
                    )
                    .await?;

                return Ok(true);
            }
            Err(why) => {
                let response = CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("Failed to join voice channel: {}", why)),
                );
                interaction.create_response(ctx, response).await?;
                return Ok(false);
            }
        }
    }

    Ok(true)
}
