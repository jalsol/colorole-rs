use color_thief::ColorFormat;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::{
    model::prelude::{
        interaction::application_command::ApplicationCommandInteraction, RoleId,
        UserId,
    },
    prelude::Context,
};

pub fn is_hex_color_code(color: &str) -> bool {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(?i)[0-9A-F]{6}$").unwrap();
    }

    PATTERN.is_match(color)
}

pub fn hex_to_dec(hex: &str) -> u64 {
    u64::from_str_radix(hex, 16).unwrap()
}

pub async fn get_member_color(
    member_id: UserId,
    database: &sqlx::SqlitePool,
) -> Result<(u64, String), sqlx::Error> {
    let member_id_str = member_id.to_string();
    let member_row = sqlx::query!(
        "SELECT r.id, r.color FROM roles r INNER JOIN members m ON m.role_id = r.id AND m.id = ?",
        member_id_str
    ).fetch_one(database).await;

    match member_row {
        Ok(record) => Ok((record.id as u64, record.color)),
        Err(error) => Err(error),
    }
}

pub fn find_color(t: image::ColorType) -> ColorFormat {
    match t {
        image::ColorType::Rgb8 => ColorFormat::Rgb,
        image::ColorType::Rgba8 => ColorFormat::Rgba,
        _ => unreachable!(),
    }
}

pub async fn set_color(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    database: &sqlx::SqlitePool,
    color: String,
) -> String {
    let guild_id = interaction.guild_id.unwrap();
    let guild_id_str = guild_id.as_u64().to_string();

    let mut member = guild_id.member(ctx, &interaction.user).await.unwrap();
    let member_id = member.user.id;
    let member_id_str = member_id.to_string();

    if let Ok((current_role_id, current_role)) =
        get_member_color(member_id, database).await
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
                    r.hoist(true).name(&color).colour(hex_to_dec(&color))
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
