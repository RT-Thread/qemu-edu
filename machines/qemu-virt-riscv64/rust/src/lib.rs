/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2024-09-20     RT-Thread    First version
 */

#![no_std]
#![allow(non_camel_case_types)]

use core::panic::PanicInfo;
use core::ffi::{c_void, c_char};

// Core modules
pub mod libc;
pub mod librt;
pub mod init;

// Include example modules when building
// Note: Examples are in separate files under src/examples/ directory
// They are included here to make their functions available to C code
include!("examples/hello.rs");
include!("examples/printf_demo.rs");
include!("examples/string_demo.rs");
include!("examples/memory_demo.rs");
include!("examples/thread_demo.rs");

// Re-export initialization function
pub use init::rust_init;

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        libc::printf(b"Rust panic occurred!\n\0".as_ptr());
    }
    loop {}
}