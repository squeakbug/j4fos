#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
