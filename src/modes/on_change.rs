use crate::{
    Config,
    util::{error::AppError, formatter::MessageFormatter, osc::OscClient},
};

use super::ModeHandler;

pub struct OnChange {
    last_track: Option<String>,
}

impl OnChange {
    pub fn new() -> Self {
        Self { last_track: None }
    }
}

impl ModeHandler for OnChange {
    fn tick(&mut self, client: &OscClient, config: &Config) -> Result<(), AppError> {
        let current =
            match MessageFormatter::format(config,  &config.on_change_message)? {
                Some(msg) => msg,
                None => return Ok(()),
            };

        let changed = match &self.last_track {
            Some(prev) if prev == &current => false,
            _ => true,
        };

        if !changed {
            return Ok(()); // nothing to do
        }

        self.last_track = Some(current.clone());

        client.send_osc(&current, config.send_immediately, config.notify_on_send)?;

        Ok(())
    }
}
