use std::{f64, process::Command};

use crate::util::error::AppError;

pub fn get_track(players: &str, user_format: &str, meta_format: &str) -> Result<String, AppError> {
    let metadata = get_metadata(players, meta_format)?;

    // Get optional position
    let position = if needs_position(user_format) {
        Some(get_position(players).unwrap_or(0.0))
    } else {
        None
    };

    // Get optional length
    let length = if needs_length(user_format) {
        Some(get_length(players).unwrap_or(0.0))
    } else {
        None
    };

    let mut result = user_format.replace("{meta}", &metadata);

    if let Some(pos) = position {
        result = result.replace("{position}", &format_time(pos));
    }

    if let Some(len) = length {
        result = result.replace("{length}", &format_time(len));
    }

    Ok(result)
}

fn needs_position(format: &str) -> bool {
    format.contains("{position}")
}

fn needs_length(format: &str) -> bool {
    format.contains("{length}")
}

pub fn get_metadata(players: &str, format: &str) -> Result<String, AppError> {
    let output = Command::new("playerctl")
        .args(["-p", players, "metadata", "--format", format])
        .output()?;

    if !output.status.success() {
        return Err(AppError::CommandFail {
            cmd: "playerctl metadata".into(),
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn get_position(players: &str) -> Result<f64, AppError> {
    let output = Command::new("playerctl")
        .args(["-p", players, "position"])
        .output()?;

    if !output.status.success() {
        return Err(AppError::CommandFail {
            cmd: "playerctl position".into(),
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    let pos = String::from_utf8(output.stdout)?.trim().parse::<f64>()?;

    Ok(pos)
}

pub fn get_length(players: &str) -> Result<f64, AppError> {
    let output = Command::new("playerctl")
        .args(["-p", players, "metadata", "--format", "{{mpris:length}}"])
        .output()?;

    if !output.status.success() {
        return Err(AppError::CommandFail {
            cmd: "playerctl mpris:length".into(),
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    // length is in microseconds
    let length_us = String::from_utf8(output.stdout)?.trim().parse::<f64>()?;

    Ok(length_us / 1_000_000.0)
}

pub fn format_time(seconds: f64) -> String {
    let total = seconds as u64;
    let mins = total / 60;
    let secs = total % 60;
    format!("{:02}:{:02}", mins, secs)
}