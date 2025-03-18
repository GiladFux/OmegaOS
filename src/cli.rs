use omega::keyboard::read_input;
use omega::print;
use omega::println;
use alloc::vec::Vec;
pub fn cli_loop() {
    loop {
        print!("> "); // CLI prompt
        if let Some(command) = read_input() {
            handle_command(command);
        }
    }
}

/// Parses the given command string and executes the corresponding action.
fn handle_command(command: &str) {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();

    if let Some(cmd) = parts.first() {
        match *cmd {
            "help" => println!("Available commands: touch <file>, rm <file>, ls, cat <file>, help, exit"),
            "exit" => println!("Thanks for using OmegaOS"),
            "touch" => {
                if let Some(filename) = parts.get(1) {
                    println!("Creating file: {}", filename);
                } else {
                    println!("Usage: touch <filename>");
                }
            }
            "rm" => {
                if let Some(filename) = parts.get(1) {
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
}