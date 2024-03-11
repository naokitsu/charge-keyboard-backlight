use std::ops::Rem;
use std::thread::sleep;
use std::time::Duration;
use crate::backlight::{Color, LEDFile, Payload, Mode};

mod backlight;
mod battery;

fn color_from_hsv(hue: f32, saturation: f32, value: f32) -> Color {
    let k = |n, h: f32| {
        (n as f32 + h / 60.0).rem(6.0)
    };

    let f = |n, h, s, v| {
        let k = k(n, h);

        let min_of_min = k.min(4.0 - k).min(1.0);

        let res = v - v * s * min_of_min.max(0.0);
        res * 255.0
    };

    Color {
        red: f(5, hue, saturation, value) as u8,
        green: f(3, hue, saturation, value) as u8,
        blue: f(1, hue, saturation, value) as u8,
    }
}

fn main() {
    let saturation = 0.8;
    let value = 1.0;

    let mut status = battery::Status::get().expect("Couldn't get battery status");
    let mut config = LEDFile::new();
    let charging_color = color_from_hsv(186.0, saturation, value);
    let charged_hue = 150.0;

    loop {
        if status.online {
            config.load(Payload {
                mode: Mode::Static(charging_color),
                save: false
            })
        } else {
            config.load(Payload {
                mode: Mode::Static(color_from_hsv(((status.charge as f32) * charged_hue) / 255.0, saturation, value)),
                save: false
            })
        }
            .expect("Failed to load payload");

        status.update();

        sleep(Duration::from_millis(10));
    }
}
