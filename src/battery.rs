use std::fs::File;
use std::io::Read;
use const_format::concatcp;
use winapi::um::winbase::LPSYSTEM_POWER_STATUS;

const PATH: &str = "/sys/class/power_supply/";
const ACAD: &str = "ACAD/";
const BAT: &str = "BAT1/";

const ONLINE: &str = "online";
const FULL: &str = "charge_full";
const NOW: &str = "charge_now";

const ONLINE_PATH: &str = concatcp!(PATH, ACAD, ONLINE);
const FULL_PATH: &str = concatcp!(PATH, BAT, FULL);
const NOW_PATH: &str = concatcp!(PATH, BAT, NOW);

pub struct Status {
    pub online: bool,
    pub charge: u8,
    max_charge: u32
}

#[derive(Copy, Clone, Debug)]
pub enum Error {
    CouldntParse,
    CouldntRead
}

impl Status {
    #[cfg(target_os = "linux")]
    fn online() -> Result<bool, Error> {
        let mut online = File::options()
            .read(true)
            .write(false)
            .open(ONLINE_PATH)
            .map_err(|_| Error::CouldntRead)?;
        let mut buf = [0u8; 1];
        online.read_exact(&mut buf)
            .map_err(|_| Error::CouldntRead)?;;
        if buf[0] == b'1' {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[cfg(target_os = "windows")]
    fn online() -> Result<bool, Error> {
        let mut status = winapi::um::winbase::SYSTEM_POWER_STATUS {
            ACLineStatus: 0,
            BatteryFlag: 0,
            BatteryLifePercent: 0,
            Reserved1: 0,
            BatteryLifeTime: 0,
            BatteryFullLifeTime: 0,
        };
        unsafe {
            if winapi::um::winbase::GetSystemPowerStatus(&mut status) == 0 {
                return Err(Error::CouldntRead)
            }
        }
        Ok(status.ACLineStatus == 1)
    }

    #[cfg(target_os = "linux")]
    fn read_battery(path: &str) -> Result<u32, Error> {
        let mut file = File::options()
            .read(true)
            .write(false)
            .open(path)
            .map_err(|_| Error::CouldntRead)?;;
        let mut buf= [0u8; 16];
        file.read(& mut buf)
            .map_err(|_| Error::CouldntRead)?;;
        let string = String::from_utf8_lossy(&buf);

        let res = string
            .split("\n")
            .next()
            .map(|x| x.parse::<u32>())
            .transpose()
            .map_err(|x| Error::CouldntParse)?
            .ok_or(Error::CouldntParse)?;

        Ok(res)
    }

    #[cfg(target_os = "windows")]
    fn read_battery() -> Result<u32, Error> {
        let mut status = winapi::um::winbase::SYSTEM_POWER_STATUS {
            ACLineStatus: 0,
            BatteryFlag: 0,
            BatteryLifePercent: 0,
            Reserved1: 0,
            BatteryLifeTime: 0,
            BatteryFullLifeTime: 0,
        };
        unsafe {
            if winapi::um::winbase::GetSystemPowerStatus(&mut status) == 0 {
                return Err(Error::CouldntRead)
            }
        }
        Ok(status.BatteryLifePercent as u32)
    }

    pub fn get() -> Result<Self, Error> {
        let online = Self::online()?;
        #[cfg(target_os = "linux")]
            let max_charge = Self::read_battery(FULL_PATH)?;
        #[cfg(target_os = "linux")]
            let current_charge = Self::read_battery(NOW_PATH)?;
        #[cfg(target_os = "windows")]
            let max_charge = 100;
        #[cfg(target_os = "windows")]
            let current_charge = Self::read_battery()?;

        Ok(Status {
            online,
            charge: (current_charge * 255 / max_charge) as u8,
            max_charge
        })
    }

    pub fn update(&mut self) -> Result<&mut Self, Error> {
        #[cfg(target_os = "linux")]
            let current_charge = Self::read_battery(NOW_PATH)?;
        #[cfg(target_os = "windows")]
            let current_charge = Self::read_battery()?;
        self.charge = (current_charge * 255 / self.max_charge) as u8;
        self.online = Self::online()?;

        Ok(self)
    }
}
