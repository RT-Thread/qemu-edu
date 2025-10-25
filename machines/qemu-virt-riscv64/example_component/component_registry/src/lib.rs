/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-10-25     foxglove     Component registry for unified component registration
 */
#![no_std]

extern crate alloc;

use macro_main::macro_main_use;
use rt_rust::param::{Param, ParamItem};
use rt_rust::println;

// 重新导出组件功能，供其他模块使用
#[cfg(feature = "enable-log")]
pub use em_component_log::*;
#[cfg(feature = "enable-log")]
use em_component_log::logging::Level;

/// 统一的组件注册入口点
/// 这个函数负责注册所有启用的组件，避免重复注册问题
#[cfg(feature = "enable-log")]
#[macro_main_use(name = "rust_component_registry", component = true, desc = "Rust component registry.")]
fn main(_param: Param) {
    println!("[logging component init] hello world");
    log!(Level::Info, "hello world");
    info!("hello world");
    warn!("hello world");
    error!("hello world");
    trace!("hello world");
    debug!("hello world");
}
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
/// 当没有启用任何组件特性时的空实现
#[cfg(not(feature = "enable-log"))]
pub extern "C" fn component_init() {
    // 空实现，确保库仍然可以被链接
}