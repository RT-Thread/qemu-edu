/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-10-10     foxglove     load library example
 */
#![no_std]

extern crate alloc;

use macro_main::rtt_main;
use rt_rust::println;
use rt_rust::get_libfn;
use core::ffi::{c_int, c_char};
use rt_rust::param::Param;

#[rtt_main(name = "rust_dl_demo", cmd = true, desc = "Rust example app.")]
fn main(_param: Param) {
    println!("\n=== Macro Demo ===");

    // 直接调用宏生成的函数
    get_libfn!("/hello.mo", "main", my_hello, ());
    my_hello();

    get_libfn!("/libmylib.mo", "rust_mylib_add", my_add, c_int, a: c_int, b: c_int);
    let s = my_add(15, 20);
    println!("my_add(15, 20) = {}", s);

    get_libfn!("/libmylib.mo", "rust_mylib_println", my_println, (), s: *const c_char);
    my_println(b"rustlib: Hello World\0".as_ptr() as *const c_char);
}
