use super::RgbController;
use anyhow::Result;

pub struct GPIOController {
    name: String,
    red_pin: u8,
    green_pin: u8,
    blue_pin: u8,
}

impl GPIOController {
    pub fn new(name: String, red_pin: u8, green_pin: u8, blue_pin: u8) -> Self {
        Self {
            name,
            red_pin,
            green_pin,
            blue_pin,
        }
    }
}

impl RgbController for GPIOController {
    fn set_color(&mut self, red: u8, green: u8, blue: u8) -> Result<()> {
        // TODO: Implement actual GPIO control
        println!(
            "[{}] Would set GPIO pins ({}, {}, {}) to RGB({}, {}, {})",
            self.name, self.red_pin, self.green_pin, self.blue_pin, red, green, blue
        );
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
} 