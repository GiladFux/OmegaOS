#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(omega::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;
mod fs;
use core::panic::PanicInfo;
use fs::file_table::FileTable;
use omega::println;
use omega::print;
use bootloader::{BootInfo, entry_point};
use omega::task::{Task, simple_executor::SimpleExecutor};
use omega::task::keyboard; 
use crate::fs::buffer::MyBlockDevice;
use x86_64::{
    structures::paging::{Page, Size4KiB, Translate},
    VirtAddr,
};

use fs::file_ops::{format_fs, create_file, write_file, read_file};
static mut STORAGE: [u8; 512 * 1024] = [0; 512 * 1024]; // 512KB storage

entry_point!(kernel_main);
fn print_hex(data: &[u8]) {
    println!("Data length: {} bytes", data.len());
    for byte in data {
        print!("{:02x} ", byte);
    }
    println!();
}
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use omega::allocator; 
    use omega::memory::{self, BootInfoFrameAllocator};
    use x86_64::structures::paging::Translate; 
    use x86_64::{structures::paging::Page, VirtAddr}; 

    println!("Hello World{}", "!");
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
    
    //  create a file
    create_file(&mut device, "file1");
    println!("created a file!");
    // Write some data to the file
    write_file(&mut device, "file1", "some data".as_bytes());
     println!("wrote to file!");

    // read the file
    let mut read_buffer = [0u8; 512]; // Buffer to store the read data
    read_file(&device, "file1", &mut read_buffer);
    print_hex(&read_buffer);


    // Run tests if in test mode
    #[cfg(test)]
    test_main();

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses())); 
    executor.run();

    println!("It did not crash!");
    omega::hlt_loop()
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
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
