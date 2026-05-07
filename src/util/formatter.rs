use crate::{Config, constants::CHARACTER_LIMIT, util::{error::AppError, player::{format_time, get_length, get_metadata, get_position}}};

pub struct MessageFormatter;

impl MessageFormatter {
    pub fn format(
        config: &Config,
        players: &str,
        template: &str,
    ) -> Result<String, AppError> {
        let meta = get_metadata(players, &config.meta_format)?;

        let mut result = template.replace("{meta}", &meta);

        if template.contains("{position}") {
            let pos = get_position(players).unwrap_or(0.0);
            result = result.replace("{position}", &format_time(pos));
        }

        if template.contains("{length}") {
            let len = get_length(players).unwrap_or(0.0);
            result = result.replace("{length}", &format_time(len));
        }

        if result.len() > CHARACTER_LIMIT  {
            eprintln!("VRChat only supports up to {CHARACTER_LIMIT} characters, your message will be trimmed!");
            result.truncate(CHARACTER_LIMIT);
            println!("Message trimmed to \"{result}\"")
        }

        Ok(result)
    }
}