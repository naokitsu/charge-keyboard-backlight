use crate::config::{Color, Config, Payload, Mode};

mod config;

fn main() {
    let mut config = Config::new();
    config.load( Payload { mode: Mode::Static(Color { red: 125, green: 0, blue: 30 }), save: true }).unwrap();
}
