use lazy_static::lazy_static;
use regex::Regex;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

fn is_hex_color_code(color: &str) -> bool {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(?i)[0-9A-F]{6}$").unwrap();
    }

    PATTERN.is_match(color)
}

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected string option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    let color;

    if let CommandDataOptionValue::String(_color) = option {
        color = _color.to_string();
    } else {
        return "Please provide a valid color".to_string();
    };

    if !is_hex_color_code(&color) {
        return format!("#{} is **not** a hex color code", color).to_string();
    }

    format!("#{} is a hex color code", color).to_string()
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
