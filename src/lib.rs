#![no_std] // Disable the standard library (suitable for bare-metal environments)
#![cfg_attr(test, no_main)] // Prevents using the standard entry point during testing
#![feature(custom_test_frameworks)] // Enables custom test framework
#![feature(abi_x86_interrupt)] // Enables x86 interrupt calling conventions
#![test_runner(crate::test_runner)] // Specifies the custom test runner function
#![reexport_test_harness_main = "test_main"] // Renames the test harness main function to `test_main`

// Module imports for serial communication, VGA output, interrupts, and GDT setup
pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;


use core::panic::PanicInfo; // Core library for     handling panic information

/// Enum to define exit codes for QEMU, used to signal success or failure in tests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10, 
    Failed = 0x11,  
}

/// Exits QEMU with the specified `QemuExitCode`
/// Communicates with QEMU via the I/O port `0xf4`
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port; // Provides low-level I/O operations

    unsafe {
        let mut port = Port::new(0xf4); // Create a port at address 0xf4
        port.write(exit_code as u32); // Write the exit code to the port
    }
}

/// Trait to make test functions runable within the custom test framework
pub trait Testable {
    fn run(&self) -> (); // Defines a method to run the test
}

/// Implementation of the `Testable` trait for all functions (`Fn`)
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>()); // Print the name of the test
        self(); // Run the test
        serial_println!("[ok]"); // Indicate the test passed
    }
}

/// Custom test runner function to execute all tests
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len()); // Print the number of tests
    for test in tests {
        test.run(); // Run each test using the `Testable` trait
    }
    exit_qemu(QemuExitCode::Success); // Exit QEMU with success code after tests complete
}

/// Custom panic handler for tests, reports errors via serial output
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n"); // Print a failed message
    serial_println!("Error: {}\n", info); // Print the panic information
    exit_qemu(QemuExitCode::Failed); // Exit QEMU with a failure code
    hlt_loop(); // Halt the CPU to prevent further execution
}

/// Entry point for `cargo test`, called when running tests in QEMU
#[cfg(test)]
#[no_mangle] // Prevents name mangling to ensure this is recognized as `_start`
pub extern "C" fn _start() -> ! {
    init(); // Initialize the system (GDT, IDT, PICs, etc.)
    test_main(); // Run all tests
    hlt_loop(); // Halt the CPU after tests complete
}

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

/// Custom panic handler for tests, delegates to `test_panic_handler`
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info) // Handle the panic using the custom handler
}

/// Initializes the system by setting up GDT, IDT, and PICs
pub fn init() {
    gdt::init(); // Initialize the Global Descriptor Table (GDT)
    interrupts::init_idt(); // Initialize the Interrupt Descriptor Table (IDT)
    unsafe { interrupts::PICS.lock().initialize() }; // Initialize the Programmable Interrupt Controllers (PICs)
    x86_64::instructions::interrupts::enable(); // Enable hardware interrupts
}

/// Halts the CPU in an infinite loop using the `hlt` instruction
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt(); // Enter a low-power state until the next interrupt
    }
}
