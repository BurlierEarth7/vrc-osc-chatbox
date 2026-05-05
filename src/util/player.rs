use std::{f64, process::Command};

use crate::util::error::AppError;

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