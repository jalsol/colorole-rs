use crate::commands;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{
    Interaction, InteractionResponseType,
};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct Handler {
    pub database: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "color" => {
                    commands::color::run(&ctx, &command, &self.database).await
                }
                "ping" => commands::ping::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content(content)
                        })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands =
            Command::set_global_application_commands(&ctx.http, |commands| {
                commands
                    .create_application_command(|command| {
                        commands::color::register(command)
                    })
                    .create_application_command(|command| {
                        commands::ping::register(command)
                    })
            })
            .await;

        if let Err(error) = commands {
            println!("Created the following slash command: {:#?}", error);
        }
    }
}
