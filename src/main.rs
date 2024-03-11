use crate::backlight::{Color, Config, Payload, Mode};

mod backlight;
mod battery;

fn main() {
    let mut config = Config::new();
    config.load( Payload { mode: Mode::Static(Color { red: 125, green: 0, blue: 100 }), save: true }).unwrap();
}
