#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(omega::test_runner)]
#![reexport_test_harness_main = "test_main"]
use omega::allocator::HEAP_SIZE;

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use omega::allocator;
    use omega::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    omega::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    test_main();
    loop {}
}
use alloc::boxed::Box;

#[test_case]
fn simple_allocation() { // verify that no allocation error occurs
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}
use alloc::vec::Vec;

#[test_case]
fn large_vec() { // verify the sum by comparing it with the formula
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() { // ensures that the allocator reuses freed memory for subsequent allocations
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    omega::test_panic_handler(info)
}