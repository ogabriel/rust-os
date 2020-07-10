#![no_std]

fn main() {}

use core::panic::PanicInfo;

/// This function is called on panic (without std, we have to implement it)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
