#![no_std]

use core::time::Duration;
use macro_main::rtt_main;
use rt_rust::param::Param;
use rt_rust::println;
use rt_rust::thread;
use rt_rust::time;

#[rtt_main(
    appname = "rust_thread_demo",
    cmd = true,
    desc = "Rust example app."
)]
fn main(_param: Param) {
    let _ = thread::Thread::new()
        .name("thread 1")
        .stack_size(1024)
        .start(move || {
            loop {
                println!("thread a will sleep 1s");
                time::sleep(Duration::new(1, 0));
            }
        });

    let _ = thread::Thread::new()
        .name("thread 2")
        .stack_size(1024)
        .start(move || {
            loop {
                println!("thread b will sleep 3s");
                time::sleep(Duration::new(3, 0));
            }
        });
}
