#![no_std]
// Bring rt-rust's println! macro into scope
use rt_rust::println;
use core::ffi::{c_char, CStr};
#[unsafe(no_mangle)]
pub extern "C" fn rust_mylib_println(s: *const c_char) {
    if s.is_null() {
        println!("");
    } else {
            let cs = unsafe {CStr::from_ptr(s)};
            match cs.to_str() {
                Ok(msg) => println!("{}", msg),
                Err(_) => println!("[invalid UTF-8]"),
            }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_mylib_add(a: usize, b: usize) -> usize {
    a + b
}