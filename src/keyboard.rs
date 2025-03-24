// Importing necessary dependencies
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static; // For lazy_static! macro

// Keyboard buffer and associated state
pub static mut INPUT_BUFFER: [u8; 256] = [0; 256];
pub static mut INPUT_INDEX: usize = 0;
pub static mut INPUT_READY: bool = false;

// Lazy static initialization of the keyboard
lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore
        ));
}

// Define keyboard port constant
pub const KEYBOARD_PORT: u16 = 0x60;

// Function to read the input from the keyboard buffer
pub fn read_input() -> Option<&'static str> {
    unsafe {
        while !INPUT_READY {} // Wait until Enter is pressed

        let input_str = core::str::from_utf8(&INPUT_BUFFER[..INPUT_INDEX]).ok()?;

        INPUT_INDEX = 0; // Reset index only after reading the input
        INPUT_READY = false;
        Some(input_str)
    }
}
