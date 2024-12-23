// Import required structures and modules for Interrupt Descriptor Table (IDT) and interrupt handling
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println; // Custom println macro for output
use crate::print;   // Custom print macro for output
use lazy_static::lazy_static; // Allows creating statics that require runtime initialization
use crate::gdt; // Import Global Descriptor Table (GDT) related functionality
use x86_64::structures::idt::PageFaultErrorCode; // Page fault error codes for handling page faults
use crate::hlt_loop; // Custom function to halt the CPU in an infinite loop

// Define a static Interrupt Descriptor Table (IDT) with lazy initialization
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new(); // Create a new IDT instance
        
        // Set the breakpoint handler
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        
        // Set the double fault handler with a custom stack using the GDT
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // Set the timer interrupt handler
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);

        // Set the keyboard interrupt handler
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);

        // Set the page fault handler
        idt.page_fault.set_handler_fn(page_fault_handler); 

        idt // Return the configured IDT
    };
}

// Function to initialize and load the IDT
pub fn init_idt() {
    IDT.load();
}

// Breakpoint interrupt handler
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    // Print the exception type and the stack frame for debugging
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// Double fault interrupt handler
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    // Panic and print the stack frame when a double fault occurs
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

// Timer interrupt handler
extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    // Print a dot to indicate timer ticks
    print!(".");

    // Notify the Programmable Interrupt Controller (PIC) that the interrupt has been handled
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

// Keyboard interrupt handler
extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    // Import required modules for handling keyboard input
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    // Define a static keyboard instance for handling input with thread safety
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(), // Initialize with ScancodeSet1
                layouts::Us104Key,   // Use the US 104-key layout
                HandleControl::Ignore // Ignore control keys
            ));
    }

    // Lock the keyboard instance and create a port for reading scancodes
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60); // Standard I/O port for keyboard input

    // Read the scancode from the keyboard buffer
    let scancode: u8 = unsafe { port.read() };

    // Process the scancode and decode the key event
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character), // Printable character
                DecodedKey::RawKey(key) => print!("{:?}", key), // Raw key (e.g., function keys)
            }
        }
    }

    // Notify the PIC that the interrupt has been handled
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

// Test function for triggering a breakpoint exception
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3(); // Trigger a breakpoint exception
}

// Import the PIC module for handling hardware interrupts
use pic8259::ChainedPics;
use spin; // Provides Mutex for synchronization

// Constants for setting PIC offsets
pub const PIC_1_OFFSET: u8 = 32; // Offset for the first PIC (avoid CPU exceptions 0-31)
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8; // Offset for the second PIC

// Static instance of ChainedPics for managing the two PICs
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe {
        // Initialize the PICs with the specified offsets
        ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
    });

// Enum for interrupt indices
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET, // Timer interrupt
    Keyboard, // Keyboard interrupt
}

impl InterruptIndex {
    // Convert the interrupt index to a u8
    fn as_u8(self) -> u8 {
        self as u8
    }

    // Convert the interrupt index to a usize
    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

// Page fault interrupt handler
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2; // Import Cr2 for reading the accessed memory address

    // Print information about the page fault
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);

    // Halt the CPU in an infinite loop
    hlt_loop();
}
