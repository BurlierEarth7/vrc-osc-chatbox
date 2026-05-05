use std::sync::Arc;

use arc_swap::ArcSwap;

use crate::{Config, util::{error::AppError, formatter::MessageFormatter, osc::OscClient}};

use super::ModeHandler;

pub struct SyncMode;

impl ModeHandler for SyncMode {
    fn tick(&mut self, client: &OscClient, config: &Config) -> Result<(), AppError> {
        let message = MessageFormatter::format(
            config,
            &config.players,
            &config.sync_message,
        )?;

        client.send_osc(
            &message,
            config.send_immediately,
            config.notify_on_send,
        )?;

        Ok(())
    }
}