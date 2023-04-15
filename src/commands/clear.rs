use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use crate::utils;

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    database: &sqlx::SqlitePool,
) -> String {
    let guild_id = interaction.guild_id.unwrap();
    let mut member = guild_id.member(ctx, &interaction.user).await.unwrap();

    utils::clear_color(&ctx, &database, guild_id, &mut member).await
}

pub fn register(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("clear")
        .description("Clear your current color")
}
