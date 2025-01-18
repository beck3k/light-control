use serde::{Serialize, Deserialize};
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const OFF: Color = Color { red: 0, green: 0, blue: 0 };
    pub const RED: Color = Color { red: 255, green: 0, blue: 0 };
    pub const WHITE: Color = Color { red: 255, green: 255, blue: 255 };
}

// CLI-friendly enum for predefined profiles
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
pub enum Profile {
    Off,
    Red,
    White,
}

// Internal representation that can handle custom colors
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ColorSetting {
    Profile(Profile),
    Custom(Color),
}

impl Profile {
    pub fn color(&self) -> Color {
        match self {
            Profile::Off => Color::OFF,
            Profile::Red => Color::RED,
            Profile::White => Color::WHITE,
        }
    }
}

impl ColorSetting {
    pub fn color(&self) -> Color {
        match self {
            ColorSetting::Profile(profile) => profile.color(),
            ColorSetting::Custom(color) => *color,
        }
    }
}

impl From<Profile> for ColorSetting {
    fn from(profile: Profile) -> Self {
        ColorSetting::Profile(profile)
    }
} 