use crate::{
    Config,
    constants::CHARACTER_LIMIT,
    util::{
        error::AppError,
        player::{
            format_time, get_length, get_metadata, get_player_status, get_position, is_playing,
            list_players,
        },
    },
};

pub struct MessageFormatter;

impl MessageFormatter {
    pub fn format(
        config: &Config,
        template: &str,
    ) -> Result<Option<String>, AppError> {
        let player = match resolve_active_player(&config.players)? {
            Some(p) => p,
            None => return Ok(None),
        };

        let meta = match try_player_call(get_metadata(&player, &config.meta_format))? {
            Some(meta) => meta,
            None => return Ok(None),
        };

        if !matches!(try_player_call(is_playing(&player))?, Some(true)) {
            return Ok(None);
        }

        let mut result = template.replace("{meta}", &meta);

        if template.contains("{position}") {
            let pos = match get_position(&player) {
                Ok(pos) => pos,
                Err(AppError::NoActivePlayers) => 0.0,
                Err(e) => return Err(e),
            };

            result = result.replace("{position}", &format_time(pos));
        }

        if template.contains("{length}") {
            let len = match get_length(&player) {
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

pub fn resolve_active_player(preferred: &str) -> Result<Option<String>, AppError> {
    let preferred: Vec<&str> = preferred.split(',').map(|s| s.trim()).collect();
    let available = list_players()?;

    let mut best_paused: Option<String> = None;

    // First pass: find Playing
    for p in &preferred {
        if !available.contains(&p.to_string()) {
            continue;
        }

        match get_player_status(p) {
            Ok(status) if status == "Playing" => {
                return Ok(Some((*p).to_string()));
            }
            Ok(status) if status == "Paused" && best_paused.is_none() => {
                best_paused = Some((*p).to_string());
            }
            _ => {}
        }
    }

    Ok(best_paused)
}
