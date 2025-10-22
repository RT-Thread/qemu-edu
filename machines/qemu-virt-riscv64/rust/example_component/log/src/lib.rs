#![no_std]

extern crate alloc;

use alloc::string::String;
use macro_main::rtt_main;
use rt_rust::logging::Level;
use rt_rust::{debug, error, info, log, println, trace, warn};
use rt_rust::param::Param;

// 组件示例：在组件阶段打印一次日志，然后注册一个 MSH 命令可手动触发
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