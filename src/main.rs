#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga_buffer;
use core::panic::PanicInfo;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Hello\tWorld{}", "!");
    #[cfg(test)]
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
