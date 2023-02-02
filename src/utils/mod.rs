use lazy_static::lazy_static;
use regex::Regex;
use serenity::model::prelude::UserId;

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
