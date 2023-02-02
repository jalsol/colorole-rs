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
    let member = guild_id.member(ctx, &interaction.user).await.unwrap();

    let avatar_url = if let Some(url) = member.user.avatar_url() {
        url
    } else {
        member.user.default_avatar_url()
    };

    let img_data = reqwest::get(avatar_url).await;

    if let Err(_) = img_data {
        return "Cannot get image.".to_string();
    }

    let img_bytes = img_data.unwrap().bytes().await;
    if let Err(_) = img_bytes {
        return "Cannot convert image to bytes.".to_string();
    }

    let img_bytes = img_bytes.unwrap();
    let image = image::load_from_memory(&img_bytes);
    if let Err(_) = image {
        return "Cannot save images to file.".to_string();
    }

    let image = image.unwrap();
    let color_type = utils::find_color(image.color());
    let colors =
        color_thief::get_palette(&image.as_bytes(), color_type, 10, 10).unwrap();

    let best_color = colors.first().unwrap();
    let color = format!(
        "{:02x}{:02x}{:02x}",
        best_color.r, best_color.g, best_color.b
    );

    utils::set_color(&ctx, &interaction, &database, color).await
}

pub fn register(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("pfp")
        .description("Choose a color that matches your profile picture")
}
