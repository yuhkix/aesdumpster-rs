mod key_dumpster;
mod other_tools;

use crate::key_dumpster::KeyDumpster;
use crate::other_tools::OtherTools;
use std::env;

fn main() {
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

            if let Some(key) = key_dumpster.get_most_likely_key() {
                match other_tools.write_key_to_file(&key, "aes.txt") {
                    Ok(_) => println!(
                        "[+] Successfully wrote the most likely key to {}!",
                        "aes.txt"
                    ),
                    Err(e) => eprintln!("[-] Error writing key to {}: {}", "aes.txt", e),
                }
            } else {
                println!(
                    "[!] Keys were found, but none met the minimum criteria for writing to {}.",
                    "aes.txt"
                );
            }
        }
    }

    other_tools.print_outro();
    other_tools.wait_for_enter();
}
