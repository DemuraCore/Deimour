use crate::lavalink_handler::LAVALINK_CLIENT;
use serenity::builder::CreateCommand;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub fn register() -> CreateCommand {
    CreateCommand::new("leave").description("Leave a voice channel")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    let guild_id: GuildId = interaction.guild_id.unwrap();
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();
    let lava_client = LAVALINK_CLIENT
        .get()
        .expect("LavalinkClient not initialized")
        .clone();

    lava_client.delete_player(guild_id).await.unwrap();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            return format!("Failed to leave voice channel: {:?}", e);
        }

        "Left voice channel".to_string()
    } else {
        "I'm not in a voice channel".to_string()
    }
}
