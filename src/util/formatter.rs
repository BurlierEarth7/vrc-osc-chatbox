use crate::{
    Config,
    constants::CHARACTER_LIMIT,
    util::{
        error::AppError,
        player::{format_time, get_length, get_metadata, get_position, is_playing},
    },
};

pub struct MessageFormatter;

impl MessageFormatter {
    pub fn format(
        config: &Config,
        players: &str,
        template: &str,
    ) -> Result<Option<String>, AppError> {
        let meta = match try_player_call(get_metadata(players, &config.meta_format))? {
            Some(meta) => meta,
            None => return Ok(None),
        };

        if !matches!(try_player_call(is_playing(players))?, Some(true)) {
            return Ok(None);
        }

        let mut result = template.replace("{meta}", &meta);

        if template.contains("{position}") {
            let pos = match get_position(players) {
                Ok(pos) => pos,
                Err(AppError::NoActivePlayers) => 0.0,
                Err(e) => return Err(e),
            };

            result = result.replace("{position}", &format_time(pos));
        }

        if template.contains("{length}") {
            let len = match get_length(players) {
                Ok(len) => len,
                Err(AppError::NoActivePlayers) => 0.0,
                Err(e) => return Err(e),
            };

            result = result.replace("{length}", &format_time(len));
        }

        if result.len() > CHARACTER_LIMIT {
            eprintln!(
                "VRChat only supports up to {CHARACTER_LIMIT} characters, your message will be trimmed!"
            );
            result.truncate(CHARACTER_LIMIT);
            println!("Message trimmed to \"{result}\"")
        }

        Ok(Some(result))
    }
}

fn try_player_call<T>(res: Result<T, AppError>) -> Result<Option<T>, AppError> {
    match res {
        Ok(v) => Ok(Some(v)),
        Err(e) if is_player_gone_error(&e) => Ok(None),
        Err(e) => Err(e),
    }
}

fn is_player_gone_error(e: &AppError) -> bool {
    match e {
        AppError::NoActivePlayers => true,
        AppError::CommandFail { stderr, .. } => {
            stderr.contains("No players found")
                || stderr.contains("No player could handle this command")
                || stderr.contains("not available")
                || stderr.contains("org.mpris.MediaPlayer2")
        }
        _ => false,
    }
}
