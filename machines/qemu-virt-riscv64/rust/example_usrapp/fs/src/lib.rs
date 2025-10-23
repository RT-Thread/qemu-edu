
/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-10-20     foxglove     Rust file opration test.
 */
#![no_std]

extern crate alloc;

use macro_main::marco_main_use;
use em_component_log::{info, error};
use rt_rust::{fs, println};
use rt_rust::param::Param;

#[marco_main_use(name = "rust_file_demo", cmd = true, desc = "Rust example app.")]
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
            info!("open test.txt ok");
            f
        }
        Err(e) => {
            error!("open error: {:?}", e);
            return;
        }
    };

    if let Err(e) = file.write_all("Hello from FS wrapper!\n") {
        error!("write_all error: {:?}", e);
        return;
    }
    info!("write_all done");

    if let Err(e) = file.flush() {
        error!("flush error: {:?}", e);
    } else {
        info!("flush ok");
    }

    match file.read_to_string() {
        Ok(s) => info!("read_back: {}", s),
        Err(e) => error!("read_to_string error: {:?}", e),
    }

    if let Err(e) = file.set_len(5) {
        error!("truncate error: {:?}", e);
    } else {
        info!("truncate to 5 ok");
    }

    if let Err(e) = file.close() {
        error!("close error: {:?}", e);
    } else {
        info!("close ok");
    }

    info!("[rust_file_test] end");
}
