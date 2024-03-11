use std::fmt::Display;
use std::fs::File;
use std::io::{Seek, Write};
use std::ptr::{null, null_mut};

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

pub struct LEDFile {
    #[cfg(target_os = "linux")]
    file: File,
    #[cfg(target_os = "windows")]
    handle: *mut winapi::ctypes::c_void,
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

impl LEDFile {
    #[cfg(target_os = "linux")]
    pub fn new() -> Self {
        Self {
            file: File::options().read(false).write(true).open(PATH).expect("Failed to open file")
        }
    }
    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        let handle = unsafe {
            winapi::um::fileapi::CreateFileW(
                b"\\\0\\\0.\0\\\0A\0T\0K\0A\0C\0P\0I\0".as_ptr() as *const u16,
                0x4000_0000 | 0x8000_0000, // Generic write & read
                2 | 1, // Share write and read
                &mut winapi::um::minwinbase::SECURITY_ATTRIBUTES {
                    nLength: 0,
                    lpSecurityDescriptor: null_mut(),
                    bInheritHandle: 0,
                },
                3, // Open existing
                0,
                null_mut()
            )
        };
        Self {
            handle,
        }
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn load(&mut self, payload: Payload) -> std::io::Result<()> {
        self.file.write(payload.to_string().as_bytes())?;
        self.file.flush()?;
        self.file.rewind()?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn load(&mut self, payload: Payload) -> std::io::Result<()> {
        let mut wmi_payload = [0u8; 20];
        let mut output = [0u8; 4];

        let (mode, red, green, blue, speed) = match payload.mode {
            Mode::Static(color) => (0, color.red, color.green, color.blue, 225),
            Mode::Breathing(color, Speed::Slow) => (1, color.red, color.green, color.blue, 225),
            Mode::Breathing(color, Speed::Medium) => (1, color.red, color.green, color.blue, 235),
            Mode::Breathing(color, Speed::Fast) => (1, color.red, color.green, color.blue, 245),
            Mode::Rainbow(Speed::Slow) => (2, 0, 0, 0, 225),
            Mode::Rainbow(Speed::Medium) => (2, 0, 0, 0, 235),
            Mode::Rainbow(Speed::Fast) => (2, 0, 0, 0, 245),
            Mode::Blinking(color) => (10, color.red, color.green, color.blue, 225),
        };

        wmi_payload[0] = 0x44;
        wmi_payload[1] = 0x45;
        wmi_payload[2] = 0x56;
        wmi_payload[3] = 0x53;

        wmi_payload[4] = 0x0c;
        wmi_payload[5] = 0;
        wmi_payload[6] = 0;
        wmi_payload[7] = 0;

        wmi_payload[8] = 0x56;
        wmi_payload[9] = 0x00;
        wmi_payload[10] = 0x10;
        wmi_payload[11] = 0x00;

        wmi_payload[12] = 0xB4;
        wmi_payload[13] = mode;
        wmi_payload[14] = red;
        wmi_payload[15] = green;
        wmi_payload[16] = blue;
        wmi_payload[17] = speed;
        wmi_payload[18] = 0x00;
        wmi_payload[19] = 0x00;

        unsafe {
            let res = winapi::um::ioapiset::DeviceIoControl(
                self.handle,
                0x22240Cu32,
                wmi_payload.as_mut_ptr() as *mut winapi::ctypes::c_void,
                20,
                output.as_mut_ptr() as *mut winapi::ctypes::c_void,
                4,
                null_mut(),
                null_mut()
            );
        }
        Ok(())
    }
}
