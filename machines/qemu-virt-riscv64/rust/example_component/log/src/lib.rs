
/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-10-10     foxglove     log component demo
 */
#![no_std]

extern crate alloc;
pub mod logging;
use macro_main::rtt_main;
use rt_rust::println;
use rt_rust::param::{Param, ParamItem};
use logging::Level;

// 组件示例：在组件阶段打印一次日志
// 打印 INFO
#[rtt_main(name = "log_info", cmd = true, desc = "Print an INFO line")]
fn cmd_log_info(_args: alloc::vec::IntoIter<ParamItem>) {
    info!("hello from rust log component");
}

// 打印 WARN
#[rtt_main(name = "log_warn", cmd = true, desc = "Print a WARN line")]
fn cmd_log_warn(_args: alloc::vec::IntoIter<ParamItem>) {
    warn!("warn from rust");
}

// 打印 ERROR
#[rtt_main(name = "log_error", cmd = true, desc = "Print an ERROR line")]
fn cmd_log_error(_args: alloc::vec::IntoIter<ParamItem>) {
    error!("error from rust");
}

#[rtt_main(name = "rust_component_demo", component = true, desc = "Rust component demo.")]
fn main(_param: Param) {
    println!("[component init] hello world");
    log!(Level::Info, "hello world");
    info!("hello world");
    warn!("hello world");
    error!("hello world");
    trace!("hello world");
    debug!("hello world");
}