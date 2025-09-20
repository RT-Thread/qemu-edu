/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2024-09-20     RT-Thread    First version
 */

// RT-Thread thread operation examples - demonstrates thread creation, synchronization and other operations

/// Thread entry function
extern "C" fn thread_entry(param: *mut c_void) {
    let id = param as usize;
    unsafe {
        libc::printf(b"[Thread %d] Started\n\0".as_ptr(), id);
        
        // 执行一些任务
        for i in 0..3 {
            libc::printf(b"[Thread %d] Working... step %d\n\0".as_ptr(), id, i + 1);
            librt::rt_thread_mdelay(100); // 休眠100ms
        }
        
        libc::printf(b"[Thread %d] Finished\n\0".as_ptr(), id);
    }
}

/// Thread creation example
#[no_mangle]
pub extern "C" fn rust_thread_create_demo() {
    unsafe {
        libc::printf(b"\n=== RT-Thread Create Demo ===\n\0".as_ptr());
        
        // 创建线程
        let thread = librt::rt_thread_create(
            b"rust_thread\0".as_ptr() as *const libc::c_char,
            thread_entry,
            1 as *mut c_void,  // Pass thread ID as parameter
            2048,              // Stack size
            20,                // Priority
            10,                // Time slice
        );
        
        if thread.is_null() {
            libc::printf(b"Failed to create thread!\n\0".as_ptr());
            return;
        }
        
        libc::printf(b"Thread created successfully\n\0".as_ptr());
        
        // 启动线程
        let ret = librt::rt_thread_startup(thread);
        if ret == librt::RT_EOK {
            libc::printf(b"Thread started successfully\n\0".as_ptr());
        } else {
            libc::printf(b"Failed to start thread: %d\n\0".as_ptr(), ret);
        }
        
        // 主线程等待一段时间
        librt::rt_thread_mdelay(500);
    }
}

/// Current thread information example
#[no_mangle]
pub extern "C" fn rust_thread_self_demo() {
    unsafe {
        libc::printf(b"\n=== Current Thread Info ===\n\0".as_ptr());
        
        let current = librt::rt_thread_self();
        if !current.is_null() {
            libc::printf(b"Current thread handle: %p\n\0".as_ptr(), current);
            
            // 获取当前tick
            let tick = librt::rt_tick_get();
            libc::printf(b"Current system tick: %u\n\0".as_ptr(), tick);
            
            // 线程让出CPU
            libc::printf(b"Yielding CPU...\n\0".as_ptr());
            librt::rt_thread_yield();
            libc::printf(b"Resumed after yield\n\0".as_ptr());
        }
    }
}

/// Thread sleep example
#[no_mangle]
pub extern "C" fn rust_thread_sleep_demo() {
    unsafe {
        libc::printf(b"\n=== Thread Sleep Demo ===\n\0".as_ptr());
        
        libc::printf(b"Sleeping for 1 second...\n\0".as_ptr());
        librt::rt_thread_mdelay(1000);
        libc::printf(b"Woke up after 1 second\n\0".as_ptr());
        
        // 使用tick方式休眠
        let ticks = librt::rt_tick_from_millisecond(500);
        libc::printf(b"Sleeping for %u ticks (500ms)...\n\0".as_ptr(), ticks);
        librt::rt_thread_delay(ticks);
        libc::printf(b"Woke up after 500ms\n\0".as_ptr());
    }
}

/// Using Rust-wrapped thread API
#[no_mangle]
pub extern "C" fn rust_thread_wrapper_demo() {
    unsafe {
        libc::printf(b"\n=== Thread Wrapper Demo ===\n\0".as_ptr());
        
        if let Some(thread) = librt::Thread::create(
            b"rust_wrap\0",
            thread_entry,
            2 as *mut c_void,
            2048,
            20,
            10,
        ) {
            libc::printf(b"Thread created using Rust wrapper\n\0".as_ptr());
            
            if thread.startup().is_ok() {
                libc::printf(b"Thread started successfully\n\0".as_ptr());
            } else {
                libc::printf(b"Failed to start thread\n\0".as_ptr());
            }
            
            // 等待线程完成
            librt::thread_sleep_ms(500);
        } else {
            libc::printf(b"Failed to create thread using wrapper\n\0".as_ptr());
        }
    }
}

/// Comprehensive thread operations demonstration
#[no_mangle]
pub extern "C" fn rust_thread_demo_all() {
    rust_thread_self_demo();
    rust_thread_sleep_demo();
    rust_thread_create_demo();
    rust_thread_wrapper_demo();
}
