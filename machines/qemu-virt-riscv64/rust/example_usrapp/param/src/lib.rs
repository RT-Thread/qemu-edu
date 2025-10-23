/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-10-10     foxglove     param test demo
 */
#![no_std]

extern crate alloc;

use alloc::string::String;
use macro_main::marco_main_use;
use rt_rust::param::Param;
use rt_rust::println;

#[marco_main_use(name = "rust_param_demo",
            cmd = true,
            app = true,
            desc = "Rust example app.")]
fn main(param: Param) {
    for i in param {
        println!("{}", String::from_utf8_lossy(&*i))
    }
}
