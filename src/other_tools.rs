use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::os::windows::ffi::OsStrExt;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Console::{
    FOREGROUND_BLUE, FOREGROUND_GREEN, FOREGROUND_INTENSITY, FOREGROUND_RED,
    SetConsoleTextAttribute,
};

pub struct RetVal {
    pub buffer: Vec<u8>,
}

pub struct OtherTools {
    pub retval: RetVal,
}

impl OtherTools {
    pub fn new() -> Self {
        Self {
            retval: RetVal { buffer: Vec::new() },
        }
    }

    pub fn print_intro(&self) {
        println!("AESDumpster-rs - Rust Implementation by yuhkix");
        println!("Based on AESDumpster by GHFear @ IllusorySoftware");
        println!(
            "Supports Unreal Engine 4.19 -> 5.3 | (Will soon support UE 4.0 - 4.18 as well)\n"
        );
    }

    pub fn print_instructions(&self) {
        println!("Usage:");
        println!("-Drag and drop Unreal Engine executables onto AESDumpster.exe.");
        println!("-Wait for the tool to finish.");
        let mut _s = String::new();
        let _ = std::io::stdin().read_line(&mut _s);
    }

    pub fn print_file_name(&self, hconsole: HANDLE, path: &str) {
        unsafe {
            let yellow = FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_INTENSITY;
            let _ = SetConsoleTextAttribute(hconsole, yellow);
        }
        println!("{}", path);
        unsafe {
            let white = FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE;
            let _ = SetConsoleTextAttribute(hconsole, white);
        }
    }

    pub fn print_outro(&self, hconsole: HANDLE) {
        unsafe {
            let green = FOREGROUND_GREEN | FOREGROUND_INTENSITY;
            let _ = SetConsoleTextAttribute(hconsole, green);
        }
        println!("Done!");
        unsafe {
            let white = FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE;
            let _ = SetConsoleTextAttribute(hconsole, white);
        }
    }

    pub fn create_exe_buffer(&mut self, filepath: &str) -> std::io::Result<()> {
        let mut file = File::open(filepath)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        self.retval.buffer = buf;
        Ok(())
    }

    pub fn wait_for_enter(&self) {
        println!("Press Enter to exit...");
        let _ = std::io::stdin().read_line(&mut String::new());
    }
}

#[allow(dead_code)]
fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
