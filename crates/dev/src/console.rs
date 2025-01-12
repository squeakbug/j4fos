use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::uart16550::putchar;

pub struct Console {}

lazy_static! {
    pub static ref GLOBAL_TTY: Mutex<Console> = Mutex::new(Console { });
}

impl Console {
    #[no_mangle]
    pub fn write_string(&mut self, str: &str) {
        unsafe {
            let mut str_ptr = str.as_ptr();
            while *str_ptr != b'\0' {
                putchar(*str_ptr);
                str_ptr = str_ptr.add(1);
            }
        }
        return;
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    GLOBAL_TTY.lock().write_fmt(args).unwrap();
}

#[cfg(not(feature = "with_std"))]
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[cfg(not(feature = "with_std"))]
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
