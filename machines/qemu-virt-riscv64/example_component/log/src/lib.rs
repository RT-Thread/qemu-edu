
/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-10-23     foxglove     log component demo
 */
#![no_std]

extern crate alloc;

#[cfg(feature = "enable-log")]
pub mod logging;
use macro_main::macro_main_use;
#[cfg(feature = "enable-log")]
use rt_rust::println;
use rt_rust::param::{Param, ParamItem};
#[cfg(feature = "enable-log")]
use crate::logging::Level;

#[cfg(feature = "enable-log")]
#[macro_main_use(name = "log_info", cmd = true, desc = "Print an INFO line")]
fn cmd_log_info(_args: alloc::vec::IntoIter<ParamItem>) {
    info!("hello from rust log component");
}

// 打印 WARN
#[cfg(feature = "enable-log")]
#[macro_main_use(name = "log_warn", cmd = true, desc = "Print a WARN line")]
fn cmd_log_warn(_args: alloc::vec::IntoIter<ParamItem>) {
    warn!("warn from rust");
}

// 打印 ERROR
#[cfg(feature = "enable-log")]
#[macro_main_use(name = "log_error", cmd = true, desc = "Print an ERROR line")]
fn cmd_log_error(_args: alloc::vec::IntoIter<ParamItem>) {
    error!("error from rust");
}

#[cfg(feature = "enable-log")]
#[macro_main_use(name = "rust_component_demo", component = true, desc = "Rust component demo.")]
fn main(_param: Param) {
    println!("[logging component init] hello world");
    log!(Level::Info, "hello world");
    info!("hello world");
    warn!("hello world");
    error!("hello world");
    trace!("hello world");
    debug!("hello world");
}

// 当 enable-log feature 未启用时，提供一个空的实现
#[cfg(not(feature = "enable-log"))]
pub extern "C" fn component_init() {
    // 空实现，确保库仍然可以被链接
}