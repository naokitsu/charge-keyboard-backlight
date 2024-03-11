use std::fmt::Display;
use std::fs::File;
use std::io::{Seek, Write};

const PATH: &str = "/sys/class/leds/asus::kbd_backlight/kbd_rgb_mode";


#[derive(Debug, Clone, Copy)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}

#[derive(Debug, Clone, Copy)]
pub struct Payload {
    pub(crate) mode: Mode,
    pub(crate) save: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Static(Color),
    Breathing(Color, Speed),
    Rainbow(Speed),
    Blinking(Color),
}


#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

pub struct Config {
    file: File,
}

impl Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let save = if self.save { "1" } else { "0" };
        let (mode, red, green, blue, speed) = match self.mode {
            Mode::Static(color) => ("0", color.red, color.green, color.blue, Speed::Slow),
            Mode::Breathing(color, speed) => ("1", color.red, color.green, color.blue, speed),
            Mode::Rainbow(speed) => ("2", 0, 0, 0, speed),
            Mode::Blinking(color) => ("10", color.red, color.green, color.blue, Speed::Slow),
        };

        write!(f, "{} {} {} {} {} {}", save, mode, red, green, blue, speed)
    }
}

impl Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Speed::Slow => write!(f, "0"),
            Speed::Medium => write!(f, "1"),
            Speed::Fast => write!(f, "2"),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            file: File::options().read(false).write(true).open(PATH).expect("Failed to open file")
        }
    }

    pub(crate) fn load(&mut self, payload: Payload) -> std::io::Result<()> {
        self.file.write(payload.to_string().as_bytes())?;;
        self.file.flush()?;
        self.file.rewind()?;
        Ok(())
    }
}
