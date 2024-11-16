use serenity::async_trait;
use serenity::prelude::*;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use colored::Colorize;
use std::env;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!(
                "Received command interaction: {}",
                command.data.name
            );

            let response = match command.data.name.as_str() {
                "help" => {
                    let content = crate::commands::help::run(&command.data.options());
                    Some(CreateInteractionResponseMessage::new().content(content))
                },
                "ping" => {
                    crate::commands::ping::run(&ctx, &command
                    ).await.ok();
                    None
                },
                "join" => {
                    crate::commands::join::run(&ctx, &command
                    ).await.ok();
                    None
                },

                "leave" => {
                    crate::commands::leave::run(&ctx, &command
                    ).await.ok();
                    None
                },

                _ => Some(CreateInteractionResponseMessage::new().content("not implemented :(".to_string())),
            };

            if let Some(response) = response {
                let builder = CreateInteractionResponse::Message(response);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name.to_string().green());

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![
                crate::commands::help::register(),
                crate::commands::ping::register(),
                crate::commands::join::register(),
                crate::commands::leave::register(),
            ])
            .await;
        if let Err(why) = commands {
            println!("Cannot set commands: {why}");
        }

        let start = crate::START.get().unwrap();
        println!("Bot started in {} ms", start.elapsed().as_millis().to_string().green());
    }
}

pub struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}