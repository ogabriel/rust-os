#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

/// Use C calling convention for this function
/// entrypoint
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize *2) = byte;
            *vga_buffer.offset(i as isize *2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on panic (without std, we have to implement it)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
