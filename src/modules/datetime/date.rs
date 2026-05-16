use crate::util::error::AppError;
use std::{process::Command, sync::OnceLock};

pub struct Date;

static HAS_DATE: OnceLock<Result<(), String>> = OnceLock::new();

impl Date {
    pub fn get_date(format: &str) -> Result<String, AppError> {
        let date = Command::new("date").arg(format).output()?;

        if !date.status.success() {
            return Err(AppError::CommandFail {
                cmd: "date".into(),
                status: date.status,
                stderr: String::from_utf8_lossy(&date.stderr).to_string(),
            });
        }

        Ok(String::from_utf8(date.stdout)?.trim().to_string())
    }

    pub fn check_has_date() -> Result<(), String> {
        HAS_DATE
            .get_or_init(
                || match Command::new("date").arg("--version").output() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("date is not installed, or is not in your PATH.".into()),
                },
            )
            .clone()
    }
}
