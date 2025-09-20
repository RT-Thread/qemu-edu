/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2024-09-20     RT-Thread    First version
 */

//! Rust component initialization module
//! 
//! Handles the initialization of the Rust component within RT-Thread

use crate::libc;

/// Component initialization function
/// This function is called during RT-Thread system initialization
#[no_mangle]
pub extern "C" fn rust_init() -> i32 {
    unsafe {
        libc::printf(b"Rust component initialized\n\0".as_ptr());
    }
    0
}