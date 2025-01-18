use super::RgbController;
use anyhow::Result;

pub struct MockController {
    name: String,
}

impl MockController {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl RgbController for MockController {
    fn set_color(&mut self, red: u8, green: u8, blue: u8) -> Result<()> {
        println!("[{}] Setting color to RGB({}, {}, {})", self.name, red, green, blue);
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
} 