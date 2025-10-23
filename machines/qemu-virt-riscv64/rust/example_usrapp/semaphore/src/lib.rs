/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-10-10     foxglove     semaphore test demo
 */
#![no_std]

extern crate alloc;

use alloc::sync::Arc;
use core::time::Duration;
use macro_main::marco_main_use;
use rt_rust::sem::Semaphore;
use rt_rust::param::Param;
use rt_rust::println;
use rt_rust::thread;
use rt_rust::time;

#[marco_main_use(name = "rust_sem_demo", cmd = true, desc = "Rust example app.")]
fn main(_param: Param) {
    let send = Arc::new(Semaphore::new().unwrap());
    let recv = send.clone();

    let _ = thread::Thread::new()
        .name("thread 1")
        .stack_size(1024)
        .start(move || {
            loop {
                time::sleep(Duration::new(1, 0));
                send.release()
            }
        });
    time::sleep(Duration::new(1, 0));
    let _ = thread::Thread::new()
        .name("thread 2")
        .stack_size(1024)
        .start(move || {
            loop {
                println!("waiting!");
                recv.take_wait_forever().unwrap();
                println!("recv a sem!")
            }
        });
}

