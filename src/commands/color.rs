use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::RoleId;
use serenity::prelude::Context;

use crate::utils;

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    database: &sqlx::SqlitePool,
) -> String {
    let option = interaction
        .data
        .options
        .get(0)
        .expect("Expected string option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    let color;

    if let CommandDataOptionValue::String(_color) = option {
        color = _color.to_string();
    } else {
        return "Please provide a valid hex color code".to_string();
    };

    if !utils::is_hex_color_code(&color) {
        return format!("#{} is **not** a hex color code", color).to_string();
    }

    let guild_id = interaction.guild_id.unwrap();
    let guild_id_str = guild_id.as_u64().to_string();

    let mut member = guild_id.member(ctx, &interaction.user).await.unwrap();
    let member_id = member.user.id;
    let member_id_str = member_id.to_string();

    if let Ok((current_role_id, current_role)) =
        utils::get_member_color(member_id, database).await
    {
        if current_role == color {
            return "Color already set.".to_string();
        }

        member.remove_role(ctx, current_role_id).await.unwrap();
    }

    let role_row = sqlx::query!(
        "SELECT id FROM roles WHERE color = ? AND guild_id = ?",
        color,
        guild_id_str
    )
    .fetch_one(database)
    .await;

    let role_id = match role_row {
        Ok(ref record) => RoleId(record.id as u64),
        Err(_) => {
            let result = guild_id
                .create_role(&ctx, |r| {
                    r.hoist(true).name(&color).colour(utils::hex_to_dec(&color))
                })
                .await;

            let role_id = result.unwrap().id;
            let role_id_str = role_id.to_string();

            let response = sqlx::query!(
                "INSERT INTO roles(id, guild_id, color) VALUES(?, ?, ?)",
                role_id_str,
                guild_id_str,
                color
            )
            .execute(database)
            .await;

            if let Err(error) = response {
                return format!("{:#?}", error);
            } else {
                println!("Inserted role id: {}", role_id_str);
            }

            role_id
        }
    };

    let role_id_str = role_id.to_string();
    member.add_role(ctx, role_id).await.unwrap();

    let response = sqlx::query!(
        "REPLACE INTO members(id, guild_id, role_id) VALUES(?, ?, ?)",
        member_id_str,
        guild_id_str,
        role_id_str
    )
    .execute(database)
    .await;

    if let Err(error) = response {
        return format!("{:#?}", error);
    }

    format!("<@{}> → #{}", member_id, color)
}

pub fn register(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("color")
        .description("Choose a color for yourself")
        .create_option(|option| {
            option
                .name("hex_code")
                .description("The hex code of your chosen color")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
