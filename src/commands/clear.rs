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
    let guild_id_str = guild_id.as_u64().to_string();

    let mut member = guild_id.member(ctx, &interaction.user).await.unwrap();
    let member_id = member.user.id;
    let member_id_str = member_id.to_string();

    let result = utils::get_member_color(member_id, database).await;

    if let Err(_) = result {
        return "No color to clear".to_string();
    }

    let role_id = result.unwrap().0;
    member.remove_role(ctx, role_id).await.unwrap();

    let response = sqlx::query!(
        "DELETE FROM members WHERE id = ? AND guild_id = ?",
        member_id_str,
        guild_id_str,
    )
    .execute(database)
    .await;

    if let Err(error) = response {
        return format!("{:#?}", error);
    }

    format!("Cleared color for <@{}>", member_id)
}

pub fn register(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("clear")
        .description("Clear your current color")
}
