
use crate::arch::drivers::early_uart;

#[macro_export]
macro_rules! early_print {
    ($($arg:tt)*) => ({
            use core::fmt::Write;
            let mut writer = $crate::arch::print::UartWriter{};
            writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}
macro_rules! early_println {
    ($fmt:expr) => (early_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (early_print!(concat!($fmt, "\n"), $($arg)*));
}

#[cfg(debug)]
macro_rules! early_debug_print {
    ($fmt:expr) => (early_println!($fmt));
    ($fmt:expr, $($arg:tt)*) => (early_println!($fmt, $($arg)*));
}

#[cfg(debug)]
macro_rules! early_debug {
    ($fmt:expr) => ($fmt);
}

#[cfg(not(debug))]
macro_rules! early_debug {
    ($fmt:expr) => ($fmt);
}

pub struct UartWriter {
}

impl ::core::fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
      unsafe {
        early_uart::UARTS[0].puts(s);
      }
        Ok(())
    }
}
