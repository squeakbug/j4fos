#![no_main]
#![no_std]

mod task;

use core::panic::PanicInfo;

use dev::uart16550::{putchar, UART_BASE};

#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    mm::init();

	dev::println!("Hello, world!");
    let uart = UART_BASE as *const u8;
	loop {
	 	putchar(uart.read_volatile());
	}
}

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
