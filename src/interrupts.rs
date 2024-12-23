use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use crate::print;
use lazy_static::lazy_static;
use crate::gdt;
use x86_64::structures::idt::PageFaultErrorCode;
use crate::hlt_loop;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);
            idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
            idt.page_fault.set_handler_fn(page_fault_handler); 

            
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    print!(".");
    unsafe { // notify the PIC that the interrupt was handled
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    // Importing necessary modules and dependencies for handling keyboard input
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    // Define a global static keyboard instance protected by a Mutex for thread safety.
    // This ensures exclusive access to the keyboard during input processing.
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(), // Initialize with ScancodeSet1 (standard set of keyboard scancodes).
                layouts::Us104Key,   // Use the US keyboard layout.
                HandleControl::Ignore // Ignore special control key events.
            ));
    }

    let mut keyboard = KEYBOARD.lock(); // Acquire a lock on the keyboard Mutex to ensure safe access.
    let mut port = Port::new(0x60); // Create a new Port instance for port 0x60 (standard I/O port for keyboard input).

    // Read the scancode from the keyboard input buffer.
    let scancode: u8 = unsafe { port.read() };

    // Add the scancode to the keyboard buffer and process the resulting key event.
    // This operation may succeed, fail, or return None if the scancode doesn't generate an event.
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        // If a valid key event is generated, process it to decode the keypress.
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                // If the key is a Unicode character (e.g., printable keys), print it.
                DecodedKey::Unicode(character) => print!("{}", character),
                // If the key is a raw key (e.g., function keys or other non-printable keys), print its debug representation.
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    // Notify the Programmable Interrupt Controller (PIC) that the keyboard interrupt has been handled.
    // This prevents the PIC from blocking further keyboard interrupts.
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

use pic8259::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32; //sets up the offset of the first PIC (since first 32 are already caught by the excepetions)
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8; //setts up the offset of second PIC

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe {
        // Initialize a new ChainedPics instance with the specified offsets for the two PICs.
        // This remaps the interrupt vectors so they do not overlap with CPU exceptions (0-31).
        // The unsafe block is necessary because this operation directly interacts with
        // hardware and assumes that the offsets provided are valid.
        ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
    });


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,

}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}
