#![no_main]
#![no_std]

use core::panic::PanicInfo;

pub const UART_BASE: usize = 0x1000_0000;

#[no_mangle]
pub unsafe fn putchar(c: u8) {
    let uart = UART_BASE as *mut u8;
    unsafe {
	    uart.write_volatile(c);
    }
}

#[no_mangle]
pub unsafe fn print(str: &str) {
    unsafe {
        let mut str_ptr = str.as_ptr();
        while *str_ptr != b'\0' {
            putchar(*str_ptr);
            str_ptr = str_ptr.add(1);
        }
    }
	return;
}
 
#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
	print("Hello world!\r\n");
    let uart = UART_BASE as *const u8;
	loop {
		putchar(uart.read_volatile());
	}
}

/// This function is called on panic.
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
