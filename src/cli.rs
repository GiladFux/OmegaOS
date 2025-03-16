use crate::keyboard::read_input;

pub fn cli_loop() {
    loop {
        print!("> "); // CLI prompt
        if let Some(command) = read_input() {
            match command.trim() {
                "help" => println!("Available commands: touch, rm, ls, cat, help"),
                "exit" => break,
                _ => println!("Unknown command: {}", command),
            }
        }
    }
}
