use futures_util::lock;
use omega::keyboard::read_input;
use omega::print;
use omega::println;
use alloc::vec::Vec;

use crate::fs::block_device::BlockDevice;
use crate::fs::file_ops::create_file;
use crate::fs::file_ops::delete_file;
use crate::fs::file_ops::read_file;
use crate::DEVICE;

pub fn cli_loop() {
    loop {
        print!("> "); // CLI prompt
        if let Some(command) = read_input() {
            match command {
                "exit" => { println!("Thanks for using OmegaOS"); return; },
                _ => handle_command(command)
            }
        }
    }
}

/// Parses the given command string and executes the corresponding action.
fn handle_command(command: &str) {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();

    if let Some(mut device_lock) = DEVICE.try_lock() {
        if let Some(device) = &mut *device_lock {
            if let Some(cmd) = parts.first() {
                match *cmd {
                    "help" => {
                        if parts.len() != 1 {
                            println!("Incorrect amount of parameters.");
                            println!("Did you mean 'help'?");
                            return;
                        }
                        println!("Available commands: touch <file>, rm <file>, wf <file>, ls, cat <file>, help, exit");
                    }
                    "touch" => {
                        if parts.len() != 2 {
                            println!("Incorrect amount of parameters.");
                            return;
                        }
                        if let Some(filename) = parts.get(1) {
                            create_file(device, filename);
                            println!("Creating file: {}", filename);
                        } else {
                            println!("Usage: touch <filename>");
                        }
                    }
                    "wf" => {
                        if parts.len() != 1 {
                            println!("Incorrect amount of parameters.");
                            return;
                        }
                        // TODO: get input to put in the file
                    }
                    "rm" => {
                        if parts.len() != 2 {
                            println!("Incorrect amount of parameters.");
                            return;
                        }
                        if let Some(filename) = parts.get(1) {
                            println!("Removing file: {}", filename);
                            delete_file(device, filename);
                        } else {
                            println!("Usage: rm <filename>");
                        }
                    }
                    "cat" => {
                        if parts.len() != 2 {
                            println!("Incorrect amount of parameters.");
                            return;
                        }
                        if let Some(filename) = parts.get(1) {
                            if let Some(data) = read_file(device, filename) {
                                if let Ok(text) = core::str::from_utf8(&data) {
                                    println!("{}", text);
                                } else {
                                    println!("File content is not valid UTF-8");
                                }
                            } else {
                                println!("File not found!");
                            }
                        } else {
                            println!("Usage: cat <filename>");
                        }
                    }
                    "ls" => {
                        if parts.len() != 1 {
                            println!("Incorrect amount of parameters.");
                            println!("Did you mean 'ls'?");
                            return;
                        }

                        let file_table = device.get_file_table().lock();

                        let files = file_table.list_files();
                        if files.is_empty() {
                            println!("No files found.");
                        } else {
                            println!("Files:");
                            for file in files {
                                println!("- {}", file);
                            }
                        }
                    },
                    _ => println!("Unknown command: {}", command)
                }
            }
        } else {
            println!("Device is not initialized.");
        }
    } else {
        println!("Device is already locked");
    }
}
