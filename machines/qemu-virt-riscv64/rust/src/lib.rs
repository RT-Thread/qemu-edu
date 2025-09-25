/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2024-09-15     foxglove     1.0 version
 * 2024-09-25     foxglove     1.1 version
 */

#![no_std]
#![allow(non_camel_case_types)]

use core::panic::PanicInfo;
use core::ffi::{c_void, c_char};

// Core modules
pub mod libc;
pub mod librt;
pub mod init;
pub mod libdl;

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

#[cfg(feature = "example_printf")]
mod example_printf {
    use crate::libc;
    use crate::librt;
    use core::ffi::{c_char, c_void};
    include!("examples/printf_demo.rs");
}
#[cfg(feature = "example_printf")]
pub use example_printf::*;

#[cfg(feature = "example_string")]
mod example_string {
    use crate::libc;
    use crate::librt;
    use core::ffi::{c_char, c_void};
    include!("examples/string_demo.rs");
}
#[cfg(feature = "example_string")]
pub use example_string::*;

#[cfg(feature = "example_memory")]
mod example_memory {
    use crate::libc;
    use crate::librt;
    use core::ffi::{c_char, c_void};
    include!("examples/memory_demo.rs");
}
#[cfg(feature = "example_memory")]
pub use example_memory::*;

#[cfg(feature = "example_thread")]
mod example_thread {
    use crate::libc;
    use crate::librt;
    use core::ffi::{c_char, c_void};
    include!("examples/thread_demo.rs");
}
#[cfg(feature = "example_thread")]
pub use example_thread::*;

#[cfg(feature = "example_dl")]
mod example_dl {
    use crate::libc;
    use crate::libdl;
    use core::ffi::{c_char, c_void, c_int};
    include!("examples/dl_demo.rs");
}
#[cfg(feature = "example_dl")]
pub use example_dl::*;

#[cfg(feature = "example_vec")]
mod example_vec {
    use crate::libc;
    use crate::librt;
    use core::{mem, ptr};
    use core::ffi::{c_char, c_void};
    include!("examples/vec_demo.rs");
}
#[cfg(feature = "example_vec")]
pub use example_vec::*;

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
