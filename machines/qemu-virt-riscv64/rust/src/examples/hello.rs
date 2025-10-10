/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2025-09-15     foxglove     1.0 version
 */

// Hello example - demonstrates basic output functionality

/// Simple hello function
#[no_mangle]
pub extern "C" fn rust_hello() {
    println!("Hello from Rust!");
}

/// Hello function with name parameter
#[no_mangle]
pub extern "C" fn rust_hello_with_name(name: *const libc::c_char) {
    if name.is_null() {
        rust_hello();
    } else {
        unsafe {
            let name_str = core::ffi::CStr::from_ptr(name);
            if let Ok(name_str) = name_str.to_str() {
                println!("Hello, {}!", name_str);
            } else {
                println!("Hello, [invalid UTF-8]!");
            }
        }
    }
}

/// Print using Rust-style string
#[no_mangle]
pub extern "C" fn rust_hello_rust_style() {
    libc::print_str("Hello from Rust (Rust style)!\n");
}