use std::{f64, process::Command};

use crate::util::error::AppError;

pub fn get_metadata(players: &str, format: &str) -> Result<String, AppError> {
    query_players(
        players,
        &["metadata", "--format", format],
        "playerctl metadata",
    )
}

pub fn get_position(players: &str) -> Result<f64, AppError> {
    let pos = query_players(players, &["position"], "playerctl position")?;
    Ok(pos.parse::<f64>()?)
}

pub fn get_length(players: &str) -> Result<f64, AppError> {
    let length_str = query_players(
        players,
        &["metadata", "--format", "mpris:length"],
        "playerctl mpris:length",
    )?;

    // length is in microseconds
    let length_us = length_str.parse::<f64>()?;

    Ok(length_us / 1_000_000.0)
}

fn query_players(players: &str, args: &[&str], cmd_name: &str) -> Result<String, AppError> {
    let query = Command::new("playerctl")
        .args(["-p", players])
        .args(args)
        .output()?;

    if !query.status.success() {
        return Err(AppError::CommandFail {
            cmd: cmd_name.into(),
            status: query.status,
            stderr: String::from_utf8_lossy(&query.stderr).to_string(),
        });
    }

    Ok(String::from_utf8(query.stdout)?.trim().to_string())
}

pub fn format_time(seconds: f64) -> String {
    let total = seconds as u64;
    let mins = total / 60;
    let secs = total % 60;
    format!("{:02}:{:02}", mins, secs)
}
