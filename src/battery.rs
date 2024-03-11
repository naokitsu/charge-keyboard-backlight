use std::fs::{File, read};
use std::io::Read;

const PATH: &str = "/sys/class/power_supply/";
const ACAD: &str = "ACAD/";
const BAT: &str = "BAT1/";

pub struct Status {
    pub online: bool,
    pub charge: u8,
    max_charge_divided_by_255: u32
}


impl Status {
    fn online() -> bool {
        let mut online = File::options()
            .read(true)
            .write(false)
            .open(concat!(PATH, ACAD, "online"))
            .expect("Failed to open current ACAD state");
        let mut buf = [0u8; 1];
        let online = online.read_exact(&mut buf)
            .expect("Failed to read current ACAD state");
        if buf[0] == b'1' {
            true
        } else {
            false
        }
    }

    fn read_battery(path: &str) -> u32 {
        let mut file = File::options()
            .read(true)
            .write(false)
            .open(concat!(PATH, BAT, "online"))
            .expect("Failed to read battery");
        let mut buf= [0u8; 16];
        file.read(& mut buf)
            .expect("Failed to read battery");
        String::from_utf8_lossy(&buf).parse::<u32>()
            .expect("Failed to parse battery value")
    }


    pub fn get() -> Self {
        let online = Self::online();
        let max_charge = Self::read_battery(concat!(PATH, BAT, "charge_full"));
        let current_charge = Self::read_battery(concat!(PATH, BAT, "charge_now"));

        Status {
            online,
            charge: ((current_charge * 255) / max_charge) as u8,
            max_charge_divided_by_255: max_charge / 255
        }
    }

    pub fn update(&mut self) -> &mut Self {
        let current_charge = Self::read_battery(concat!(PATH, BAT, "charge_now"));
        self.charge = (current_charge / self.max_charge_divided_by_255) as u8;
        self
    }
}
