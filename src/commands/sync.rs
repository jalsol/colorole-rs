use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use crate::utils::{is_hex_color_code, set_color};

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    database: &sqlx::SqlitePool,
) -> String {
    let guild_id = interaction.guild_id.unwrap();
    let roles = guild_id.roles(&ctx).await.unwrap();
    let members = guild_id.members(&ctx, Some(1000), None).await.unwrap();
    let mut ret = String::new();

    for member in members {
        let role_ids = &member.roles;
        let role = role_ids.iter().find(|&id| {
            let role = roles.get(&id).unwrap();
            is_hex_color_code(&role.name)
        });

        if role.is_none() {
            continue;
        }

        let role = role.unwrap();
        let role_str = role.to_string();
        let color = &roles.get(&role).unwrap().name;

        let guild_id = interaction.guild_id.unwrap();
        let guild_id_str = guild_id.to_string();

        let response = sqlx::query!(
            "REPLACE INTO roles(id, guild_id, color) VALUES(?, ?, ?)",
            role_str,
            guild_id_str,
            color
        )
        .execute(database)
        .await;

        if let Err(error) = response {
            return format!("{:#?}", error);
        } else {
            println!("Inserted role id: {}, color is {}", role_str, color);
        }

        let member_id_str = member.user.id.to_string();

        let response = sqlx::query!(
            "REPLACE INTO members(id, guild_id, role_id) VALUES(?, ?, ?)",
            member_id_str,
            guild_id_str,
            role_str
        )
        .execute(database)
        .await;

        if let Err(error) = response {
            return format!("{:#?}", error);
        }

        ret += &format!("{},{}\n", member.user.id, color);
    }

    println!("{}", ret);

    "ok".to_string()
}

pub fn register(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("sync")
        .description("Sync current roles in the server")
}
