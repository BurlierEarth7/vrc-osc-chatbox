use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Sync,
    Swap
}

impl Mode {
    pub fn toggle(self) -> Self {
        match self {
            Mode::Sync => Mode::Swap,
            Mode::Swap => Mode::Sync
        }
    }
}
