
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
use alloc::string::String;
use macro_main::rtt_main;
use rt_rust::println;
use rt_rust::param::Param;
use logging::Level;

// 组件示例：在组件阶段打印一次日志
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