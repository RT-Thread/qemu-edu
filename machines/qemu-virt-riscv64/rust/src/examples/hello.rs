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
pub extern "C" fn rust_hello_with_name(name: *const c_char) {
    if name.is_null() {
        rust_hello();
        return;
    }
    let name_str = unsafe { CStr::from_ptr(name) }
        .to_str()
        .unwrap_or("[invalid UTF-8]");
    println!("Hello, {}!", name_str);
}

/// Print using Rust-style string
#[no_mangle]
pub extern "C" fn rust_hello_rust_style() {
    print!("Hello from Rust (Rust style)!\n");
}