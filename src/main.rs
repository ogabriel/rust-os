#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

/// Use C calling convention for this function
/// entrypoint
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World {}", "!");

    loop {}
}

/// This function is called on panic (without std, we have to implement it)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
