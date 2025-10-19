#![no_std]

extern crate alloc;

use alloc::string::String;
use macro_main::rtt_main;
use rt_rust::param::Param;
use rt_rust::println;

#[rtt_main(appname = "rust_param_demo",
            cmd = true,
            run = true,
            desc = "Rust example app.")]
fn main(param: Param) {
    for i in param {
        println!("{}", String::from_utf8_lossy(&*i))
    }
}
