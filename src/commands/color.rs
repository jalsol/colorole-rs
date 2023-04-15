use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
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
    let mut member = guild_id.member(ctx, &interaction.user).await.unwrap();

    utils::set_color(&ctx, &database, guild_id, &mut member, color).await
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
