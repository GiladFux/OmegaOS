#![no_std]
#![no_main]
mod vga_buffer;
use core::panic::PanicInfo;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Hello\tWorld{}", "!");
    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}