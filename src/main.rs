#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(omega::test_runner)]
#![reexport_test_harness_main = "test_main"]
use spin::Mutex;
extern crate alloc;
mod fs;
mod cli;
use core::panic::PanicInfo;
use fs::file_table::FileTable;
use omega::println;
use bootloader::{BootInfo, entry_point};
use crate::fs::buffer::MyBlockDevice;
use crate::cli::cli_loop;

use fs::file_ops::format_fs;
static mut STORAGE: [u8; 512 * 1024] = [0; 512 * 1024]; // 512KB storage

entry_point!(kernel_main);


static DEVICE: Mutex<Option<MyBlockDevice>> = Mutex::new(None); // Use Mutex to make it mutable and safe


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to OmegaOS!");
    use omega::allocator; 
    use omega::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr; 
    omega::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    // Initialize the allocator
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    let mut device = unsafe { MyBlockDevice::new(&mut STORAGE) };
    // Format the filesystem
    format_fs(&mut device);
    // Lock the DEVICE mutex and set it
    let mut device_lock = DEVICE.lock();
    *device_lock = Some(device); // Initialize the global device

    drop(device_lock);  // drop the lock to allow other parts to acquire it

    cli_loop();

    // Run tests if in test mode
    #[cfg(test)]
    test_main();

    omega::hlt_loop()
}



/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    omega::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    omega::test_panic_handler(info);
}
