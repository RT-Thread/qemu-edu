/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       foxglove
 * 2025-09-15     foxglove     1.0 version
 * 2025-09-25     foxglove     1.1 version
 */

#![no_std]
#![allow(non_camel_case_types)]
#[warn(unused_imports)]

pub mod bindings;
pub mod api;
pub mod malloc;
pub mod init;
pub mod out;
pub mod puts;
pub mod thread;
pub mod mutex;
pub mod sem;
pub mod queue;
pub mod time;
extern crate alloc;
use core::panic::PanicInfo;
use core::ffi::c_void;
use crate::malloc::RttAlloc;
use crate::bindings::libc;
/// Global allocator instance
#[global_allocator]
static ALLOCATOR: RttAlloc = RttAlloc;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RTTError {
    ThreadStartupErr,
    MutexTakeTimeout,
    SemaphoreTakeTimeout,
    QueueSendTimeout,
    QueueReceiveTimeout,
    OutOfMemory,

    DeviceNotFound,
    DeviceOpenFailed,
    DeviceCloseFailed,
    DeviceReadFailed,
    DeviceWriteFailed,
    DeviceTransFailed,
    DeviceConfigFailed,
    DeviceSetRxCallBackFailed,
    DeviceSetTxCallBackFailed,

    FuncUnDefine,
}

pub type RTResult<T> = Result<T, RTTError>;

// Example modules are gated by Cargo features so they can be toggled from Kconfig
// Each module re-exports its `#[no_mangle]` extern functions when enabled
#[cfg(feature = "example_hello")]
mod example_hello {
    use crate::bindings::libc;
    use crate::bindings::librt;
    use crate::{print, println};
    // use core::ffi::{c_char, c_void};
    include!("examples/hello.rs");
}
#[cfg(feature = "example_hello")]
pub use example_hello::*;

#[cfg(feature = "example_printf")]
mod example_printf {
    use crate::bindings::libc;
    use crate::bindings::librt;
    use core::ffi::{c_char, c_void};
    use crate::{print, println};
    include!("examples/printf_demo.rs");
}
#[cfg(feature = "example_printf")]
pub use example_printf::*;

#[cfg(feature = "example_string")]
mod example_string {
    use crate::bindings::libc;
    use crate::bindings::librt; 
    use core::ffi::{c_char, c_void};
    use crate::{print, println};
    include!("examples/string_demo.rs");
}
#[cfg(feature = "example_string")]
pub use example_string::*;

#[cfg(feature = "example_memory")]
mod example_memory {
    use crate::bindings::libc;
    use crate::bindings::librt;
    use core::ffi::{c_char, c_void};
    use crate::{print, println};
    include!("examples/memory_demo.rs");
}
#[cfg(feature = "example_memory")]
pub use example_memory::*;

#[cfg(feature = "example_thread")]
mod example_thread {
    use crate::bindings::libc;
    use crate::bindings::librt;
    use core::ffi::{c_char, c_void};
    include!("examples/thread_demo.rs");
}
#[cfg(feature = "example_thread")]
pub use example_thread::*;

#[cfg(feature = "example_mutex")]
mod example_mutex {
    use crate::bindings::libc;
    use crate::bindings::librt;
    use core::ffi::{c_char, c_void};
    use crate::{print, println};
    include!("examples/mutex_demo.rs");
}
#[cfg(feature = "example_mutex")]
pub use example_mutex::*;

#[cfg(feature = "example_sem")]
mod example_sem {
    use crate::bindings::libc;
    use crate::bindings::librt;
    use core::ffi::{c_char, c_void};
    use crate::{print, println};
    use crate::sem::Semaphore;
    use crate::thread::Thread;
    use alloc::format;
    use alloc::sync::Arc;
    use alloc::string::String;
    include!("examples/sem_demo.rs");
}
#[cfg(feature = "example_sem")]
pub use example_sem::*;

#[cfg(feature = "example_vec")]
mod example_vec {
    use crate::bindings::libc;
    use crate::bindings::librt;
    use core::{mem, ptr};
    use core::ffi::{c_char, c_void};
    use core::alloc::{GlobalAlloc, Layout};
    use alloc::vec::Vec;
    use crate::{print, println};
    include!("examples/vec_demo.rs");
}
#[cfg(feature = "example_vec")]
pub use example_vec::*;

#[cfg(feature = "example_dl")]
mod example_dl {
    use crate::bindings::libc;
    use crate::bindings::libdl;
    use core::ffi::{c_char, c_void, c_int};
    include!("examples/dlmodule_demo.rs");
}
#[cfg(feature = "example_dl")]
pub use example_dl::*;

#[cfg(feature = "example_mq")]
mod example_mq {
    use crate::bindings::libc;
    use crate::bindings::librt;
    use core::ffi::{c_char, c_void};
    use crate::{print, println};
    use crate::queue::Queue;
    use crate::time::sleep;
    include!("examples/mq_demo.rs");
}
#[cfg(feature = "example_mq")]
pub use example_mq::*;

// Re-export initialization function
pub use init::rust_init;

// Panic handler
#[panic_handler]
#[inline(never)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("{:}", info);
    loop {}
}
