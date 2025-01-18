pub mod rgb_controller;

use serde::{Deserialize, Serialize};
pub use rgb_controller::profiles::{Profile, Color, ColorSetting};

#[derive(Serialize, Deserialize)]
pub struct RgbCommand {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Serialize, Deserialize)]
pub enum Command {
    SetColor(RgbCommand),
    SetProfile(ColorSetting),
}

// Re-export everything needed by the binary
pub use rgb_controller::RgbController;
pub use rgb_controller::mote::MoteController; 