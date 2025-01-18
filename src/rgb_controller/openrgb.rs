use super::RgbController;
use anyhow::Result;

pub struct OpenRGBController {
    name: String,
    address: String,
    port: u16,
}

impl OpenRGBController {
    pub fn new(name: String, address: String, port: u16) -> Self {
        Self { name, address, port }
    }
}

impl RgbController for OpenRGBController {
    fn set_color(&mut self, red: u8, green: u8, blue: u8) -> Result<()> {
        // TODO: Implement OpenRGB protocol
        println!("[{}] Would set OpenRGB color to RGB({}, {}, {}) at {}:{}",
            self.name, red, green, blue, self.address, self.port);
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
} 