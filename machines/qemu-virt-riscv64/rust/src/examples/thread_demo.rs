/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 202-09-25     foxglove     test thread operations
 */

// RT-Thread thread operation examples - demonstrates safe Rust thread interface

/// Thread creation example using Thread::spawn
#[no_mangle]
pub extern "C" fn rust_thread_create_demo() {
    println!("\n=== Rust Thread Create Demo ===");
    
    // 使用Thread::spawn创建线程
    match Thread::spawn(
        String::from("rust_thread"),
        2048,  // Stack size
        20,    // Priority
        10,    // Time slice
        move || {
            println!("[Thread 1] Started");
            
            // 执行一些任务
            for i in 0..3 {
                println!("[Thread 1] Working... step {}", i + 1);
                Thread::ms_delay(100); // 休眠100ms
            }
            
            println!("[Thread 1] Finished");
        }
    ) {
        Ok(_thread) => {
            println!("Thread created and started successfully");
            
            // 主线程等待一段时间
            Thread::ms_delay(500);
        }
        Err(e) => {
            println!("Failed to create thread: {:?}", e);
        }
    }
}

/// Current thread operations example
#[no_mangle]
pub extern "C" fn rust_thread_self_demo() {
    println!("\n=== Current Thread Operations ===");
    
    // 线程让出CPU
    println!("Yielding CPU...");
    Thread::r#yield();
    println!("Resumed after yield");
}

/// Thread sleep example using Thread delay methods
#[no_mangle]
pub extern "C" fn rust_thread_sleep_demo() {
    println!("\n=== Thread Sleep Demo ===");
    
    println!("Sleeping for 1 second...");
    Thread::ms_delay(1000);
    println!("Woke up after 1 second");
    
    // 使用tick方式休眠
    println!("Sleeping for 50 ticks...");
    Thread::delay(50);
    println!("Woke up after 50 ticks");
}

/// Using ThreadBuilder API
#[no_mangle]
pub extern "C" fn rust_thread_wrapper_demo() {
    println!("\n=== ThreadBuilder Demo ===");
    
    // 使用ThreadBuilder创建线程
    match Thread::new()
        .name("rust_builder")
        .stack_size(4096)
        .priority(15)
        .ticks(20)
        .start(move || {
            println!("[Thread 2] Started with ThreadBuilder");
            
            for i in 0..5 {
                println!("[Thread 2] Builder task step {}", i + 1);
                Thread::ms_delay(80);
            }
            
            println!("[Thread 2] ThreadBuilder task completed");
        }) {
        Ok(_thread) => {
            println!("Thread created using ThreadBuilder successfully");
            
            // 等待线程完成
            Thread::ms_delay(600);
        }
        Err(e) => {
            println!("Failed to create thread using ThreadBuilder: {:?}", e);
        }
    }
}

/// Multiple threads concurrent execution demo
#[no_mangle]
pub extern "C" fn rust_thread_concurrent_demo() {
    println!("\n=== Concurrent Threads Demo ===");
    
    let mut threads = alloc::vec::Vec::new();
    
    // 创建多个并发线程
    for i in 0..3 {
        match Thread::spawn(
            format!("worker_{}", i),
            2048,
            20 + i as u8,  // 不同优先级
            10,
            move || {
                println!("[Worker {}] Starting concurrent task", i);
                
                for step in 0..4 {
                    println!("[Worker {}] Step {} executing", i, step + 1);
                    Thread::ms_delay(150 + i * 50); // 不同的延时
                }
                
                println!("[Worker {}] Concurrent task completed", i);
            }
        ) {
            Ok(thread) => {
                threads.push(thread);
                println!("Worker thread {} created successfully", i);
            }
            Err(e) => {
                println!("Failed to create worker thread {}: {:?}", i, e);
            }
        }
    }
    
    println!("All worker threads created, waiting for completion...");
    
    // 等待所有线程完成
    Thread::ms_delay(2000);
    
    println!("Concurrent demo completed");
}

/// Comprehensive thread operations demonstration
#[no_mangle]
pub extern "C" fn rust_thread_demo_all() {
    rust_thread_self_demo();
    rust_thread_sleep_demo();
    rust_thread_create_demo();
    rust_thread_wrapper_demo();
    rust_thread_concurrent_demo();
}
