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
#![feature(alloc_error_handler)]
#![feature(linkage)]
#![feature(core_intrinsics)]
#![allow(dead_code)]

pub mod bindings;
pub mod api;
pub mod libloader;
pub mod malloc;
pub mod init;
pub mod out;
pub mod puts;
pub mod thread;
pub mod mutex;
pub mod sem;
pub mod queue;
pub mod time;
pub mod param;
extern crate alloc;
use crate::malloc::RttAlloc;

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
    use crate::{print, println};
    use core::ffi::{c_char, CStr};
    include!("examples/hello.rs");
}
#[cfg(feature = "example_hello")]
pub use example_hello::*;

#[cfg(feature = "example_printf")]
mod example_printf {
    use alloc::vec::Vec;
    use crate::println;
    include!("examples/printf_demo.rs");
}
#[cfg(feature = "example_printf")]
pub use example_printf::*;

#[cfg(feature = "example_memory")]
mod example_memory {
    use crate::bindings::libc;
    use crate::bindings::librt;
    use core::ffi::c_void;
    use crate::{print, println};
    include!("examples/memory_demo.rs");
}
#[cfg(feature = "example_memory")]
pub use example_memory::*;

#[cfg(feature = "example_thread")]
mod example_thread {
    use crate::thread::Thread;
    use crate::println;
    use alloc::string::String;
    use alloc::format;
    include!("examples/thread_demo.rs");
}
#[cfg(feature = "example_thread")]
pub use example_thread::*;

#[cfg(feature = "example_mutex")]
mod example_mutex {
    use crate::mutex::Mutex;
    use crate::thread::Thread;
    use crate::println;
    use alloc::string::String;
    use alloc::format;
    use alloc::sync::Arc;
    use alloc::vec::Vec;
    include!("examples/mutex_demo.rs");
}
#[cfg(feature = "example_mutex")]
pub use example_mutex::*;

#[cfg(feature = "example_sem")]
mod example_sem {
    use crate::println;
    use crate::sem::Semaphore;
    use crate::thread::Thread;
    use alloc::format;
    use alloc::sync::Arc;
    use alloc::string::String;
    include!("examples/sem_demo.rs");
}
#[cfg(feature = "example_sem")]
pub use example_sem::*;

#[cfg(feature = "example_mq")]
mod example_mq {
    use crate::println;
    use crate::queue::Queue;
    include!("examples/mq_demo.rs");
}
#[cfg(feature = "example_mq")]
pub use example_mq::*;

#[cfg(feature = "example_dl")]
mod example_dl {
    use crate::{println, get_libfn};
    use crate::libloader;
    use core::ffi::{c_int, c_void, c_char};
    include!("examples/dlmodule_demo.rs");
}
#[cfg(feature = "example_dl")]
pub use example_dl::*;

#[cfg(feature = "bench_test")]
mod example_bench {
    use crate::println;
    use crate::time;
    include!("examples/rust_bench.rs");
}
#[cfg(feature = "bench_test")]
pub use example_bench::*;

// Re-export initialization function
pub use init::rust_init;

fn panic_on_atomic_context(s: &str) {
    use core::intrinsics::unlikely;
    use crate::api::is_irq_context;
    if unlikely(is_irq_context()) {
        panic!("In irq context {}", s);
    }
}

#[panic_handler]
#[inline(never)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("{:}", info);
    __rust_panic()
}

#[linkage = "weak"]
#[no_mangle]
fn __rust_panic() -> ! {
    loop {}
}