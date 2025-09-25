/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       foxglove
 * 2024-09-15     foxglove     1.0 version
 */

//! C standard library API bindings
//! 
//! Provides Rust wrappers for standard C library functions

pub use core::ffi::{c_char, c_int, c_void, c_uint, c_long};

// ============== stdio functions ==============
extern "C" {
    pub fn printf(fmt: *const u8, ...) -> c_int;
    pub fn sprintf(str: *mut c_char, fmt: *const c_char, ...) -> c_int;
    pub fn snprintf(str: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn putchar(c: c_int) -> c_int;
}

/// Print string (Rust style)
pub fn print_str(s: &str) {
    for byte in s.bytes() {
        unsafe { putchar(byte as c_int) };
    }
}

/// Print null-terminated string
pub fn print_cstr(s: &[u8]) {
    unsafe { printf(s.as_ptr()) };
}