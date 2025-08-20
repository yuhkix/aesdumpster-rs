mod key_dumpster;
mod other_tools;

use crate::key_dumpster::KeyDumpster;
use crate::other_tools::OtherTools;

use std::env;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Console::GetStdHandle;
use windows::Win32::System::Console::STD_OUTPUT_HANDLE;

#[cfg(debug_assertions)]
fn debug_main(hconsole: HANDLE) {
    // Adjust this path to a local executable if needed
    let exe_path = r"Z:\\Exes\\NotProtected\\SessionGame-Win64-Shipping.exe";

    let mut other_tools = OtherTools::new();
    other_tools.print_file_name(hconsole, exe_path);

    if other_tools.create_exe_buffer(exe_path).is_err() {
        return;
    }
    if other_tools.retval.buffer.is_empty() {
        eprintln!("retval.buffer is empty.");
        return;
    }

    let mut key_dumpster = KeyDumpster::new();
    if !key_dumpster.find_aes_keys(&other_tools.retval.buffer) {
        println!("There were no keys to be found or a problem occurred.");
    } else {
        key_dumpster.print_key_information();
    }

    other_tools.print_outro(hconsole);
    other_tools.wait_for_enter();
}

#[cfg(not(debug_assertions))]
fn release_main(hconsole: HANDLE) {
    let mut other_tools = OtherTools::new();

    other_tools.print_intro();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        other_tools.print_instructions();
        return;
    }

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }
        other_tools.print_file_name(hconsole, arg);
        if other_tools.create_exe_buffer(arg).is_err() {
            return;
        }
        if other_tools.retval.buffer.is_empty() {
            eprintln!("retval.buffer is empty.");
            break;
        }

        let mut key_dumpster = KeyDumpster::new();
        if !key_dumpster.find_aes_keys(&other_tools.retval.buffer) {
            println!("There were no keys to be found or a problem occurred.");
        } else {
            key_dumpster.print_key_information();
        }
    }

    other_tools.print_outro(hconsole);
    other_tools.wait_for_enter();
}

fn main() {
    let hconsole = unsafe { GetStdHandle(STD_OUTPUT_HANDLE).expect("GetStdHandle failed") };

    #[cfg(debug_assertions)]
    debug_main(hconsole);

    #[cfg(not(debug_assertions))]
    release_main(hconsole);
}
