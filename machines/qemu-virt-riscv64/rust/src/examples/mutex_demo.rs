/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2024-09-25     foxglove     test mutex operations
 */

// RT-Thread mutex operation examples - demonstrates safe Rust mutex interface

use crate::mutex::Mutex;
use crate::thread::Thread;
use alloc::string::String;
use alloc::format;
use alloc::sync::Arc;
use alloc::vec::Vec;

/// Basic mutex creation and lock/unlock example
#[no_mangle]
pub extern "C" fn rust_mutex_basic_demo() {
    println!("\n=== Rust Mutex Basic Demo ===");
    
    // 创建一个基础 mutex
    match Mutex::new(42i32) {
        Ok(mutex) => {
            println!("Mutex created successfully with initial value: 42");
            
            // 获取锁并修改值
            match mutex.lock() {
                Ok(mut guard) => {
                    println!("Lock acquired, current value: {}", *guard);
                    *guard = 100;
                    println!("Value modified to: {}", *guard);
                    // guard 在这里自动释放锁
                }
                Err(e) => {
                    println!("Failed to acquire lock: {:?}", e);
                }
            }
            
            // 再次获取锁验证值
            match mutex.lock() {
                Ok(guard) => {
                    println!("Lock acquired again, value is now: {}", *guard);
                }
                Err(e) => {
                    println!("Failed to acquire lock again: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to create mutex: {:?}", e);
        }
    }
}

/// Named mutex example
#[no_mangle]
pub extern "C" fn rust_mutex_named_demo() {
    println!("\n=== Named Mutex Demo ===");
    
    // 创建一个命名的 mutex
    match Mutex::new_with_name(String::from("Hello Mutex"), "demo_mutex") {
        Ok(mutex) => {
            println!("Named mutex 'demo_mutex' created successfully");
            
            match mutex.lock() {
                Ok(mut guard) => {
                    println!("Current string: {}", *guard);
                    guard.push_str(" - Modified!");
                    println!("Modified string: {}", *guard);
                }
                Err(e) => {
                    println!("Failed to acquire lock: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to create named mutex: {:?}", e);
        }
    }
}

/// Try lock with timeout example
#[no_mangle]
pub extern "C" fn rust_mutex_trylock_demo() {
    println!("\n=== Mutex Try Lock Demo ===");
    
    match Mutex::new(0u32) {
        Ok(mutex) => {
            println!("Mutex created for try_lock demo");
            
            // 立即尝试获取锁（不等待）
            match mutex.try_lock(0) {
                Ok(mut guard) => {
                    println!("Lock acquired immediately, value: {}", *guard);
                    *guard += 1;
                    println!("Value incremented to: {}", *guard);
                }
                Err(e) => {
                    println!("Failed to acquire lock immediately: {:?}", e);
                }
            }
            
            // 尝试获取锁，最多等待100ms
            match mutex.try_lock(100) {
                Ok(mut guard) => {
                    println!("Lock acquired with 100ms timeout, value: {}", *guard);
                    *guard += 10;
                    println!("Value incremented to: {}", *guard);
                }
                Err(e) => {
                    println!("Failed to acquire lock within 100ms: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to create mutex: {:?}", e);
        }
    }
}

/// Mutex demonstration with immediate locking
#[no_mangle]
pub extern "C" fn rust_mutex_atomic_demo() {
    println!("\n=== Mutex Immediate Lock Demo ===");
    
    // 创建 mutex 并立即锁定
    match Mutex::new(0i32) {
        Ok(mutex) => {
            println!("Mutex created successfully");
            
            match mutex.lock() {
                Ok(mut guard) => {
                    println!("Lock acquired, value: {}", *guard);
                    *guard = 999;
                    println!("Value set to: {}", *guard);
                }
                Err(e) => {
                    println!("Failed to acquire lock: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to create mutex: {:?}", e);
        }
    }
}

/// Multi-threaded mutex contention example
#[no_mangle]
pub extern "C" fn rust_mutex_concurrent_demo() {
    println!("\n=== Concurrent Mutex Demo ===");
    
    // 创建共享的 mutex
    match Mutex::new_with_name(0i32, "shared_counter") {
        Ok(shared_mutex) => {
            let mutex_arc = Arc::new(shared_mutex);
            println!("Shared mutex created for concurrent access");
            
            let mut threads = Vec::new();
            
            // 创建多个线程来竞争同一个 mutex
            for i in 0..3 {
                let mutex_clone = Arc::clone(&mutex_arc);
                
                match Thread::spawn(
                    format!("mutex_worker_{}", i),
                    2048,
                    20 + i as u8,
                    10,
                    move || {
                        println!("[Worker {}] Starting mutex operations", i);
                        
                        for step in 0..3 {
                            // 尝试获取锁
                            match mutex_clone.lock() {
                                Ok(mut guard) => {
                                    let old_value = *guard;
                                    println!("[Worker {}] Step {}: Got lock, value = {}", i, step + 1, old_value);
                                    
                                    // 模拟一些工作
                                    Thread::ms_delay(50);
                                    
                                    *guard += 1;
                                    println!("[Worker {}] Step {}: Incremented value to {}", i, step + 1, *guard);
                                    
                                    // guard 在这里自动释放锁
                                }
                                Err(e) => {
                                    println!("[Worker {}] Step {}: Failed to get lock: {:?}", i, step + 1, e);
                                }
                            }
                            
                            // 短暂休眠后继续
                            Thread::ms_delay(30);
                        }
                        
                        println!("[Worker {}] Completed all mutex operations", i);
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
            Thread::ms_delay(1500);
            
            // 检查最终值
            match mutex_arc.lock() {
                Ok(guard) => {
                    println!("Final counter value: {} (expected: 9)", *guard);
                }
                Err(e) => {
                    println!("Failed to check final value: {:?}", e);
                }
            }
            
            println!("Concurrent mutex demo completed");
        }
        Err(e) => {
            println!("Failed to create shared mutex: {:?}", e);
        }
    }
}

/// Different mutex types demonstration
#[no_mangle]
pub extern "C" fn rust_mutex_types_demo() {
    println!("\n=== Mutex Types Demo ===");
    
    // String Mutex
    println!("--- String Mutex ---");
    match Mutex::new_with_name(String::from("Hello Mutex"), "string_mutex") {
        Ok(string_mutex) => {
            println!("String mutex created successfully");
            match string_mutex.lock() {
                Ok(mut guard) => {
                    println!("String mutex locked: {}", *guard);
                    guard.push_str(" [Modified]");
                    println!("String mutex data: {}", *guard);
                }
                Err(e) => {
                    println!("Failed to lock string mutex: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to create string mutex: {:?}", e);
        }
    }
    
    // Numeric Mutex
    println!("--- Numeric Mutex ---");
    match Mutex::new_with_name(12345u64, "numeric_mutex") {
        Ok(numeric_mutex) => {
            println!("Numeric mutex created successfully");
            match numeric_mutex.try_lock(100) {
                Ok(mut guard) => {
                    println!("Numeric mutex locked: {}", *guard);
                    *guard *= 2;
                    println!("Numeric mutex data: {}", *guard);
                }
                Err(e) => {
                    println!("Failed to lock numeric mutex: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to create numeric mutex: {:?}", e);
        }
    }
}

/// Comprehensive mutex operations demonstration
#[no_mangle]
pub extern "C" fn rust_mutex_demo_all() {
    rust_mutex_basic_demo();
    rust_mutex_named_demo();
    rust_mutex_trylock_demo();
    rust_mutex_atomic_demo();
    rust_mutex_types_demo();
    rust_mutex_concurrent_demo();
}