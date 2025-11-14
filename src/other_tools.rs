use std::fs::File;
use std::io::Error as IoError;
use std::io::{self, Read, Write};

#[cfg(windows)]
use windows::Win32::System::Console::{
    FOREGROUND_BLUE, FOREGROUND_GREEN, FOREGROUND_INTENSITY, FOREGROUND_RED, GetStdHandle,
    STD_OUTPUT_HANDLE, SetConsoleTextAttribute,
};

#[cfg(unix)]
use crossterm::{
    execute,
    style::{Color, ResetColor, SetForegroundColor},
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

    #[allow(dead_code)]
    pub fn print_intro(&self) {
        println!("AESDumpster-rs");
        println!("Based on AESDumpster by GHFear @ IllusorySoftware");
        println!("Supports Unreal Engine 4.19 -> 5.3\n");
    }

    #[allow(dead_code)]
    pub fn print_instructions(&self) {
        println!("Usage:");
        println!("- Pass one or more Unreal Engine executables as arguments.");
        println!("  Example: ./aesdumpster /path/to/game1.exe /path/to/game2.exe");
        println!("- Wait for the tool to finish.");
    }

    pub fn print_file_name(&self, path: &str) {
        #[cfg(windows)]
        unsafe {
            let hconsole = GetStdHandle(STD_OUTPUT_HANDLE).unwrap();
            let yellow = FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_INTENSITY;
            let white = FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE;
            let _ = SetConsoleTextAttribute(hconsole, yellow);
            println!("{}", path);
            let _ = SetConsoleTextAttribute(hconsole, white);
        }

        #[cfg(unix)]
        {
            let mut stdout = io::stdout();
            execute!(stdout, SetForegroundColor(Color::Yellow)).unwrap();
            println!("{}", path);
            execute!(stdout, ResetColor).unwrap();
        }
    }

    pub fn print_outro(&self) {
        #[cfg(windows)]
        unsafe {
            let hconsole = GetStdHandle(STD_OUTPUT_HANDLE).unwrap();
            let green = FOREGROUND_GREEN | FOREGROUND_INTENSITY;
            let white = FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE;
            let _ = SetConsoleTextAttribute(hconsole, green);
            println!("Done!");
            let _ = SetConsoleTextAttribute(hconsole, white);
        }

        #[cfg(unix)]
        {
            let mut stdout = io::stdout();
            execute!(stdout, SetForegroundColor(Color::Green)).unwrap();
            println!("Done!");
            execute!(stdout, ResetColor).unwrap();
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
        print!("Press Enter to exit...");
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(&mut String::new());
    }

    pub fn write_key_to_file(&self, key: &str, filepath: &str) -> Result<(), IoError> {
        let mut file = File::create(filepath)?;
        writeln!(file, "0x{}", key)?;
        Ok(())
    }
}
