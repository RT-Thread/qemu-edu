/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-01-XX     foxglove     Modular macro library for RT-Thread Rust
 */

//! RT-Thread Rust 宏库
//! 
//! 提供模块化的宏定义，用于简化 RT-Thread Rust 应用开发：
//! 
//! ## 主要宏
//! 
//! - `rt_thread_main!` - 程序入口宏，标记 Rust 的 main 函数
//! - `rt_component_export!` - 导出组件初始化入口的宏
//! - `rt_app_export!` - 导出应用初始化入口的宏
//! - `msh_cmd_export!` - 导出 shell 命令的宏
//! 
//! ## 使用示例
//! 
//! ### 主函数入口
//! ```rust
//! use rt_thread_main::rt_thread_main;
//! 
//! #[rt_thread_main(name = "my_app")]
//! fn main(args: vec::IntoIter<rt_rust::param::ParamItem>) {
//!     rt_rust::println!("Hello RT-Thread!");
//! }
//! ```
//! 
//! ### 组件导出
//! ```rust
//! use rt_thread_main::rt_component_export;
//! 
//! #[rt_component_export(name = "my_component")]
//! fn init_component() {
//!     rt_rust::println!("Component initialized");
//! }
//! ```
//! 
//! ### 应用导出
//! ```rust
//! use rt_thread_main::rt_app_export;
//! 
//! #[rt_app_export(name = "my_app")]
//! fn init_app() {
//!     rt_rust::println!("App initialized");
//! }
//! ```
//! 
//! ### Shell 命令导出
//! ```rust
//! use rt_thread_main::msh_cmd_export;
//! 
//! #[msh_cmd_export(name = "hello", desc = "Say hello")]
//! fn hello_cmd(args: vec::IntoIter<rt_rust::param::ParamItem>) {
//!     rt_rust::println!("Hello from command!");
//! }
//! ```

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

// 导入模块化的宏定义
mod macros;

// 新的模块化宏定义
/// RT-Thread 主函数入口宏
#[proc_macro_attribute]
pub fn rt_thread_main(args: TokenStream, input: TokenStream) -> TokenStream {
    macros::main::rt_thread_main(args, input)
}

/// RT-Thread 组件导出宏
#[proc_macro_attribute]
pub fn rt_component_export(args: TokenStream, input: TokenStream) -> TokenStream {
    macros::component::rt_component_export(args, input)
}

/// RT-Thread 应用导出宏
#[proc_macro_attribute]
pub fn rt_app_export(args: TokenStream, input: TokenStream) -> TokenStream {
    macros::app::rt_app_export(args, input)
}

/// MSH 命令导出宏
#[proc_macro_attribute]
pub fn msh_cmd_export(args: TokenStream, input: TokenStream) -> TokenStream {
    macros::cmd::msh_cmd_export(args, input)
}
