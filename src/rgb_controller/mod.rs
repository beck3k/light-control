use anyhow::Result;
use std::any::Any;

pub trait RgbController: Send + Sync {
    fn set_color(&mut self, red: u8, green: u8, blue: u8) -> Result<()>;
    fn name(&self) -> &str;
    fn as_any(&mut self) -> &mut dyn Any;
}

pub mod profiles;
pub mod mote; 