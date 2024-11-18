use serenity::all::{Colour, CreateEmbed};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> CreateEmbed {
    CreateEmbed::new()
        .title("Help")
        .color(Colour::DARK_GREEN)
        .description("This is a help command")
        .field("ping", "Get latency bot to discord gateway", false)
        .field("stop", "Stop the music", false)
        .field("skip", "Skip the current song", false)
        .field("join", "Join the bot to a voice channel", false)
        .field("leave", "Leave the voice channel", false)
        .field("play", "Play a song", false)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("help").description("A help command")
}
