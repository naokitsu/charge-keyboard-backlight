use std::thread::sleep;
use std::time::Duration;
use crate::backlight::{Color, LEDFile, Payload, Mode};

mod backlight;
mod battery;

fn main() {
    let mut status = battery::Status::get();
    let mut config = LEDFile::new();
    let payload =  Payload { mode: Mode::Static(Color { red: 255, green: 64, blue: 128 }), save: false };
    let payload_charging =  Payload { mode: Mode::Static(Color { red: 32, green: 160, blue: 255 }), save: false };
    loop {
        if status.online {
            config.load(payload_charging)
        } else {
            config.load(payload)
        }
            .expect("Failed to load payload");

        status.update();
        sleep(Duration::from_millis(500));
    }
    println!("Bye!");
}
