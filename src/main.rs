// src/main.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(omega::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use omega::println;
use bootloader::{BootInfo, entry_point};
use x86_64::{
    structures::paging::{Page, Size4KiB, Translate},
    VirtAddr,
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use omega::memory::{self, BootInfoFrameAllocator, EmptyFrameAllocator};
    use x86_64::structures::paging::Translate; // Ensure Translate is in scope
    use x86_64::{structures::paging::Page, VirtAddr}; // new import

    println!("Hello World{}", "!");
    omega::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    // Run tests if in test mode
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
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
