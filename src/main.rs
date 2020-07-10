#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic (without std, we have to implement it)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]

/// Use C calling convention for this function
/// entrypoint
pub extern "C" fn _start() -> ! {
    loop {}
}
