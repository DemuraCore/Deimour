use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "its help".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("help").description("A help command")
}