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

    println!("Hello World{}", "!");
    omega::init();

    // Initialize the BootInfoFrameAllocator
    let frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // Initialize the mapper
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    // Choose the page at virtual address 0 (requires existing page tables)
    let page = Page::<Size4KiB>::containing_address(VirtAddr::new(0));

    // Write the string `New!` to the screen through the new mapping
    // Test address translation (optional)
    let addresses = [
        // The identity-mapped VGA buffer page
        0xb8000,
        // Some code page
        0x201008,
        // Some stack page
        0x0100_0020_1a10,
        // Virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        match mapper.translate_addr(virt) {
            Some(phys) => println!("{:?} -> {:?}", virt, phys),
            None => println!("{:?} -> Not Mapped", virt),
        }
    }

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
