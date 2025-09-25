/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       foxglove
 * 2024-09-15     foxglove     1.0 version
 */

#![no_std]
#![allow(non_camel_case_types)]

use core::panic::PanicInfo;

// Core modules
pub mod libc;
pub mod librt;
pub mod init;

// Example modules are gated by Cargo features so they can be toggled from Kconfig
// Each module re-exports its `#[no_mangle]` extern functions when enabled
#[cfg(feature = "example_hello")]
mod example_hello {
    use crate::libc;
    use crate::librt;
    use core::ffi::{c_char, c_void};
    include!("examples/hello.rs");
}
#[cfg(feature = "example_hello")]
pub use example_hello::*;

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
