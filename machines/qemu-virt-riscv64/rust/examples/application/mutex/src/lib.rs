/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-10-10     foxglove     mutex test demo
 */
#![no_std]

extern crate alloc;

use alloc::sync::Arc;
use core::time::Duration;
use macro_main::macro_main_use;
use rt_rust::mutex::Mutex;
use rt_rust::param::Param;
use rt_rust::println;
use rt_rust::thread;
use rt_rust::time;

#[macro_main_use(name = "rust_mutex_demo", cmd = true, desc = "Rust example app.")]
fn main(_param: Param) {
    let counter = Arc::new(Mutex::new(0).unwrap());
    let run = move || loop {
        time::sleep(Duration::new(2, 0));
        {
            let mut c = counter.lock().unwrap();
            *c += 1;
            println!("{}", *c);
        }
    };

    let _ = thread::Thread::new()
        .name("thread 1")
        .stack_size(2048)
        .start(run.clone());
    time::sleep(Duration::new(1, 0));
    let _ = thread::Thread::new()
        .name("thread 2")
        .stack_size(2048)
        .start(run.clone());
}
