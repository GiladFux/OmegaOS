// Importing necessary dependencies
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame; // For InterruptStackFrame
use lazy_static::lazy_static; // For lazy_static! macro
use crate::{print, println};
use crate::interrupts::{PICS, InterruptIndex}; // If you need to use PICS (Programmable Interrupt Controller)

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

// The interrupt handler function for the keyboard
pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(KEYBOARD_PORT);

    // Reading the scancode from the port
    let scancode: u8 = unsafe { port.read() };

    // Process the scancode
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            unsafe {
                match key {
                    DecodedKey::Unicode('\n') => {
                        INPUT_READY = true; // Mark input as ready when Enter is pressed
                    }
                    DecodedKey::Unicode('\x08') => { // Backspace
                        if INPUT_INDEX > 0 {
                            INPUT_INDEX -= 1;
                            print!("\x08 \x08"); // Clear character on screen
                        }
                    }
                    DecodedKey::Unicode(character) => {
                        if INPUT_INDEX < INPUT_BUFFER.len() - 1 {
                            INPUT_BUFFER[INPUT_INDEX] = character as u8;
                            INPUT_INDEX += 1;
                            print!("{}", character); // Echo typed character
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Notify the PIC that the interrupt is handled
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

// Function to read the input from the keyboard buffer
pub fn read_input() -> Option<&'static str> {
    unsafe {
        while !INPUT_READY {} // Wait until Enter is pressed

        let input_str = core::str::from_utf8(&INPUT_BUFFER[..INPUT_INDEX]).ok()?;
        
        INPUT_INDEX = 0;
        INPUT_READY = false;
        Some(input_str)
    }
}
