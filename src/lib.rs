#![no_std]
#![no_main]

use core::panic::PanicInfo;

// idc bout this rn
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
