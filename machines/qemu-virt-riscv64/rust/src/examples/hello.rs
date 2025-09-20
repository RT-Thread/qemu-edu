/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2024-09-20     RT-Thread    First version
 */

// Hello example - demonstrates basic output functionality

/// Simple hello function
#[no_mangle]
pub extern "C" fn rust_hello() {
    unsafe {
        libc::printf(b"Hello from Rust!\n\0".as_ptr());
    }
}

/// Hello function with name parameter
#[no_mangle]
pub extern "C" fn rust_hello_with_name(name: *const libc::c_char) {
    if name.is_null() {
        rust_hello();
    } else {
        unsafe {
            libc::printf(b"Hello, \0".as_ptr());
            libc::printf(name as *const u8);
            libc::printf(b"!\n\0".as_ptr());
        }
    }
}

/// Print using Rust-style string
#[no_mangle]
pub extern "C" fn rust_hello_rust_style() {
    libc::print_str("Hello from Rust (Rust style)!\n");
}