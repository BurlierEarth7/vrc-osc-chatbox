use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    #[default]
    Sync,
    Swap
}