pub const UART_BASE: usize = 0x1000_0000;

#[no_mangle]
pub unsafe fn putchar(c: u8) {
    let uart = UART_BASE as *mut u8;
    unsafe {
	    uart.write_volatile(c);
    }
}