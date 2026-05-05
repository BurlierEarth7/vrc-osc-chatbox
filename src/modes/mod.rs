use crate::{Config, util::{error::AppError, osc::OscClient}};

pub(crate) mod sync;
pub(crate) mod on_change;

pub trait ModeHandler {
    fn tick(&mut self, client: &OscClient, config: &Config) -> Result<(), AppError>;
}