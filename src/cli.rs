use omega::keyboard::read_input;
use omega::print;
use omega::println;
use alloc::vec::Vec;

use crate::fs::file_ops::create_file;
use crate::fs::file_ops::delete_file;
use crate::DEVICE;

pub fn cli_loop() {
    loop {
        print!("> "); // CLI prompt
        if let Some(command) = read_input() {
            match command
            {
                "exit" => {println!("Thanks for using OmegaOS"); return;},
                _ => handle_command(command)
            }
            
        }
    }
}

/// Parses the given command string and executes the corresponding action.
fn handle_command(command: &str) {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();

    // Lock the DEVICE mutex to safely access it
    let mut device_lock = DEVICE.lock();
    if parts.len() != 2
    {
        println!("Incorrect amount of parameters.");
    }
    if let Some(device) = &mut *device_lock {
        // Process the command
        if let Some(cmd) = parts.first() {
            match *cmd {
                "help" => println!("Available commands: touch <file>, rm <file>, ls, cat <file>, help, exit"),
                "touch" => {
                    if let Some(filename) = parts.get(1) {
                        create_file(device, filename); // Use the device here
                        println!("Creating file: {}", filename);
                    } else {
                        println!("Usage: touch <filename>");
                    }
                }
                "rm" => {
                    if let Some(filename) = parts.get(1) {
                        delete_file(device, file_table, file_name);
                        println!("Removing file: {}", filename);
                    } else {
                        println!("Usage: rm <filename>");
                    }
                }
                "cat" => {
                    if let Some(filename) = parts.get(1) {
                        println!("Displaying contents of: {}", filename);
                    } else {
                        println!("Usage: cat <filename>");
                    }
                }
                _ => println!("Unknown command: {}", command),
            }
        }
    } else {
        println!("Device is not initialized.");
    }
}
