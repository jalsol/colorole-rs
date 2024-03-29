use crate::commands;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct Handler {
    pub database: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            command.defer(&ctx.http).await.unwrap();

            let content = match command.data.name.as_str() {
                "color" => {
                    commands::color::run(&ctx, &command, &self.database).await
                }
                "clear" => {
                    commands::clear::run(&ctx, &command, &self.database).await
                }
                "sync" => commands::sync::run(&ctx, &command, &self.database).await,
                "pfp" => commands::pfp::run(&ctx, &command, &self.database).await,
                "ping" => commands::ping::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            command
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.content(content)
                })
                .await
                .unwrap();
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
                        commands::clear::register(command)
                    })
                    .create_application_command(|command| {
                        commands::sync::register(command)
                    })
                    .create_application_command(|command| {
                        commands::pfp::register(command)
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
