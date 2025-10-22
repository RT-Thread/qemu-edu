
/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-10-10     foxglove     Rust file opration test.
 */
#![no_std]

extern crate alloc;

use macro_main::rtt_main;
use rt_rust::{fs, println};
use rt_rust::param::Param;

#[rtt_main(name = "rust_file_demo", cmd = true, desc = "Rust example app.")]
fn main(_param: Param) {
    println!("[rust_file_test] start");

    let mut file = match fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .truncate(true)
        .open("test.txt")
    {
        Ok(f) => {
            println!("open test.txt ok");
            f
        }
        Err(e) => {
            println!("open error: {:?}", e);
            return;
        }
    };

    if let Err(e) = file.write_all("Hello from FS wrapper!\n") {
        println!("write_all error: {:?}", e);
        return;
    }
    println!("write_all done");

    if let Err(e) = file.flush() {
        println!("flush error: {:?}", e);
    } else {
        println!("flush ok");
    }

    match file.read_to_string() {
        Ok(s) => println!("read_back: {}", s),
        Err(e) => println!("read_to_string error: {:?}", e),
    }

    if let Err(e) = file.set_len(5) {
        println!("truncate error: {:?}", e);
    } else {
        println!("truncate to 5 ok");
    }

    if let Err(e) = file.close() {
        println!("close error: {:?}", e);
    } else {
        println!("close ok");
    }

    println!("[rust_file_test] end");
}
