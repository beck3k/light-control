use super::RgbController;
use super::profiles::{Color, ColorSetting};
use anyhow::{Result, Context};
use pyo3::prelude::*;
use std::time::Duration;
use tokio::time::sleep;
use std::any::Any;
use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::env;

pub struct MoteController {
    name: String,
    py_mote: PyObject,
    current_state: [[Color; 16]; 4],  // [channel][pixel]
}

fn get_python_path() -> Option<String> {
    env::var("VIRTUAL_ENV").ok().map(|venv_path| {
        if cfg!(target_os = "macos") {
            format!("{}/lib/python3.11/site-packages", venv_path)
        } else {
            format!("{}/lib/python3/site-packages", venv_path)
        }
    })
}

impl MoteController {
    pub fn new(name: String) -> Result<Self> {
        if let Some(site_packages) = get_python_path() {
            env::set_var("PYTHONPATH", site_packages);
        }
        println!("PYTHONPATH: {}", env::var("PYTHONPATH").unwrap_or_default());

        Python::with_gil(|py| {
            // Import the mote module
            let mote_module = PyModule::import(py, "mote")
                .context("Failed to import mote module. Is it installed?")?;
            
            // Create a new Mote object
            let mote = mote_module.getattr("Mote")?.call0()?;
            
            // Configure all 4 strips
            for port in 1..=4 {
                mote.call_method1("configure_channel", (port, 16, false))?;
            }
            
            // Clear any existing state
            mote.call_method0("clear")?;
            mote.call_method0("show")?;
            
            // Store the configured Mote object
            let py_mote = mote.into_py(py);
            
            Ok(Self { 
                name, 
                py_mote,
                current_state: [[Color::OFF; 16]; 4],
            })
        })
    }

    async fn set_pixel(&mut self, channel: usize, pixel: usize, color: Color) -> Result<()> {
        Python::with_gil(|py| {
            let mote = self.py_mote.as_ref(py);
            mote.call_method1(
                "set_pixel",
                (channel + 1, pixel, color.red, color.green, color.blue)
            )?;
            mote.call_method0("show")?;
            Ok::<_, anyhow::Error>(())
        })?;
        self.current_state[channel][pixel] = color;
        Ok(())
    }

    pub async fn transition_to(&mut self, setting: ColorSetting) -> Result<()> {
        let target_color = setting.color();
        let delay = Duration::from_millis(50);

        // Create a sequence of pixels to update
        let mut pixels: Vec<(usize, usize)> = Vec::new();
        for channel in 0..4 {
            for pixel in 0..16 {
                pixels.push((channel, pixel));
            }
        }

        // Create a thread-safe RNG using from_entropy()
        let mut rng = StdRng::from_entropy();
        pixels.shuffle(&mut rng);

        // Update each pixel with a delay
        for (channel, pixel) in pixels {
            self.set_pixel(channel, pixel, target_color).await?;
            sleep(delay).await;
        }

        Ok(())
    }
}

impl RgbController for MoteController {
    fn set_color(&mut self, red: u8, green: u8, blue: u8) -> Result<()> {
        let color = Color { red, green, blue };
        Python::with_gil(|py| {
            let mote = self.py_mote.as_ref(py);
            
            // Update all pixels
            for channel in 1..=4 {
                for pixel in 0..16 {
                    mote.call_method1(
                        "set_pixel",
                        (channel, pixel, color.red, color.green, color.blue)
                    )?;
                }
            }
            
            mote.call_method0("show")?;
            Ok::<_, anyhow::Error>(())
        })?;

        // Update state
        for channel in 0..4 {
            for pixel in 0..16 {
                self.current_state[channel][pixel] = color;
            }
        }
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Drop for MoteController {
    fn drop(&mut self) {
        // Clear the LEDs when the controller is dropped
        let _ = Python::with_gil(|py| {
            let mote = self.py_mote.as_ref(py);
            let _ = mote.call_method0("clear");
            let _ = mote.call_method0("show");
        });
    }
} 