/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-09-15     foxglove     1.0 version
 * 2025-09-25     foxglove     1.1 version
 * 2025-10-10     foxglove     latest version
 */

//! RT-THREAD & RUST
//!
//! A simple and easy-to-use system support library
//! that provides basic functions and FS, NET and DEVICE.
//!
//! You can use this library on embedded devices that support rt-thread

#![no_std]
#![feature(alloc_error_handler)]
#![feature(linkage)]
#![allow(dead_code)]

pub extern crate alloc;
mod puts;

#[doc = "Alloc by rt-thread"]
#[global_allocator]
static GLOBAL: malloc::RttAlloc = malloc::RttAlloc;
pub mod api;
mod bindings;

pub mod init;
pub mod malloc;
pub mod mutex;
pub mod out;
pub mod logging;
pub mod param;
pub mod queue;
pub mod sem;
pub mod thread;
pub mod time;
pub mod libloader;

mod prelude;
pub use prelude::v1::*;

// Re-export initialization function
pub use init::rust_init;

// TODO: review this enum
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

fn panic_on_atomic_context(s: &str) {
    use crate::api::is_irq_context;
    if is_irq_context() {
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
#[unsafe(no_mangle)]
fn __rust_panic() -> ! {
    loop {}
}