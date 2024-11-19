use crate::lavalink_handler::LAVALINK_CLIENT;
use crate::utils::responds_build;
use crate::Error;
use serenity::builder::{
    CreateCommand, CreateEmbed, CreateEmbedFooter, CreateInteractionResponseMessage,
};
use serenity::model::{colour, prelude::*};
use serenity::prelude::*;
use std::time::Instant;
use sysinfo::{ProcessorExt, System, SystemExt}; // Add this line

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
    let start = Instant::now();
    let lavalink = LAVALINK_CLIENT
        .get()
        .expect("LavalinkClient not initialized")
        .clone();

    let guild_id = interaction.guild_id.unwrap();

    let lavalink_stats = lavalink.request_info(guild_id).await?;
    let lavalink_players = lavalink.request_stats(guild_id).await?;

    // Gather system information
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let cpu_usage = sys.global_processor_info().cpu_usage();

    // Create an embed message with the stats
    let duration: std::time::Duration = start.elapsed();
    let embed = CreateEmbed::default()
        .title("Bot Stats")
        .colour(colour::Colour::DARK_GREEN)
        .field("Total Memory", format!("{} KB", total_memory), true)
        .field("Used Memory", format!("{} KB", used_memory), true)
        .field("Total Swap", format!("{} KB", total_swap), true)
        .field("Used Swap", format!("{} KB", used_swap), true)
        .field("CPU Usage", format!("{:.2}%", cpu_usage), true)
        .field(
            "Playing Players",
            lavalink_players.playing_players.to_string(),
            true,
        )
        .field(
            "Lavalink Version",
            format!("v{:?}", lavalink_stats.version.major),
            false,
        )
        .footer(CreateEmbedFooter::new(format!(
            "Deimour took: {}ms to calculate this",
            duration.as_millis()
        )));

    // Send the embed message as a response
    responds_build::send(
        ctx,
        interaction,
        CreateInteractionResponseMessage::new().embed(embed),
    )
    .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("stats").description("Give stats about the bot")
}
