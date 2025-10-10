//! Provide the output function of debugging serial port
use crate::puts::puts;
use crate::bindings::librt::rt_kputs;
use core::fmt::{self, Write};

struct StdOut;

impl fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        fn rtt_kputs(s: *const u8) {
            unsafe { 
                rt_kputs(s);
            }
        }
        puts(s, rtt_kputs);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    unsafe {
        StdOut.write_fmt(args).unwrap_unchecked();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::out::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ({
        $crate::out::_print(format_args!("\n"));
    });
    ($($arg:tt)*) => ({
        $crate::out::_print(format_args!("{}\n", format_args!($($arg)*)));
    });
}