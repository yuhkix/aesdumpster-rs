mod key_dumpster;
mod other_tools;

use crate::key_dumpster::KeyDumpster;
use crate::other_tools::OtherTools;
use std::env;

#[cfg(debug_assertions)]
fn debug_main() {
    let exe_path = r"path/to/your/test/executable"; // <-- IMPORTANT: Update this path for your Linux/Windows system

    let mut other_tools = OtherTools::new();
    other_tools.print_file_name(exe_path);

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

    other_tools.print_outro();
    other_tools.wait_for_enter();
}

#[cfg(not(debug_assertions))]
fn release_main() {
    let mut other_tools = OtherTools::new();
    other_tools.print_intro();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        other_tools.print_instructions();
        return;
    }

    for arg in args.iter().skip(1) {
        other_tools.print_file_name(arg);
        if other_tools.create_exe_buffer(arg).is_err() {
            continue;
        }
        if other_tools.retval.buffer.is_empty() {
            eprintln!("retval.buffer is empty for file: {}", arg);
            continue;
        }

        let mut key_dumpster = KeyDumpster::new();
        if !key_dumpster.find_aes_keys(&other_tools.retval.buffer) {
            println!("No keys found in: {}", arg);
        } else {
            key_dumpster.print_key_information();
        }
    }

    other_tools.print_outro();
    other_tools.wait_for_enter();
}

fn main() {
    #[cfg(debug_assertions)]
    debug_main();

    #[cfg(not(debug_assertions))]
    release_main();
}
