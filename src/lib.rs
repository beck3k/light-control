pub mod rgb_controller;

use serde::{Deserialize, Serialize};
pub use rgb_controller::profiles::{Profile, Color, ColorSetting};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RgbCommand {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    SetColor(RgbCommand),
    SetProfile(ColorSetting),
    Reconnect,
}

#[derive(Debug, Clone)]
pub struct MoteState {
    pub current_profile: Option<ColorSetting>,
    pub last_color: Option<RgbCommand>,
}

impl Default for MoteState {
    fn default() -> Self {
        Self {
            current_profile: None,
            last_color: None,
        }
    }
}

// Re-export everything needed by the binary
pub use rgb_controller::RgbController;
pub use rgb_controller::mote::MoteController; 