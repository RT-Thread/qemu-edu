/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2025-09-20     foxglove    test memory operations
 */

// Memory operation examples - demonstrates memory allocation, copy, set and other operations

/// Basic addition function (for testing)
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplication function (for testing)
#[no_mangle]
pub extern "C" fn rust_multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Memory copy test
#[no_mangle]
pub extern "C" fn rust_memcpy_test(dest: *mut u8, src: *const u8, size: usize) -> bool {
    if dest.is_null() || src.is_null() || size == 0 {
        return false;
    }
    unsafe {
        libc::memcpy(dest as *mut c_void, src as *const c_void, size);
    }
    true
}

/// Memory set example
#[no_mangle]
pub extern "C" fn rust_memset_demo() {
    let mut buffer: [u8; 32] = [0; 32];
    
    unsafe {
        // 设置前16字节为'A'
        libc::memset(buffer.as_mut_ptr() as *mut c_void, b'A' as i32, 16);
        
        // 设置后16字节为'B'
        let second_half = buffer.as_mut_ptr().add(16) as *mut c_void;
        libc::memset(second_half, b'B' as i32, 16);
        
        // 打印结果
        print!("Buffer after memset: ");
        for i in 0..32 {
            print!("{}", buffer[i] as char);
        }
        println!();
    }
}

/// Memory comparison example
#[no_mangle]
pub extern "C" fn rust_memcmp_demo() {
    let buf1 = [1u8, 2, 3, 4, 5];
    let buf2 = [1u8, 2, 3, 4, 5];
    let buf3 = [1u8, 2, 3, 4, 6];
    
    unsafe {
        let result1 = libc::memcmp(
            buf1.as_ptr() as *const c_void,
            buf2.as_ptr() as *const c_void,
            5,
        );
        println!("memcmp(buf1, buf2) = {} (should be 0)", result1);
        
        let result2 = libc::memcmp(
            buf1.as_ptr() as *const c_void,
            buf3.as_ptr() as *const c_void,
            5,
        );
        println!("memcmp(buf1, buf3) = {} (should be negative)", result2);
    }
}

/// Standard C library malloc/free example
#[no_mangle]
pub extern "C" fn rust_malloc_demo() {
    println!("\n=== Standard C malloc/free Demo ===");
    
    unsafe {
        // 分配内存
        let size = 100;
        let ptr = libc::malloc(size);
        
        if ptr.is_null() {
            println!("malloc failed!");
            return;
        }
        
        println!("Allocated {} bytes at {:p}", size, ptr);
        
        // 使用内存
        libc::memset(ptr, 0x42, size);
        
        // 释放内存
        libc::free(ptr);
        println!("Memory freed");
    }
}

/// RT-Thread memory allocation example
#[no_mangle]
pub extern "C" fn rust_rt_malloc_demo() {
    println!("\n=== RT-Thread malloc/free Demo ===");
    
    unsafe {
        // 使用RT-Thread的内存分配
        let size = 64;
        if let Some(ptr) = librt::rt_safe_malloc(size) {
            println!("RT-Thread allocated {} bytes at {:p}", size, ptr);
            
            // 使用内存
            let buffer = ptr as *mut u8;
            for i in 0..size {
                *buffer.add(i) = (i as u8) & 0xFF;
            }
            
            // 验证数据
            let mut sum = 0u32;
            for i in 0..size {
                sum += *buffer.add(i) as u32;
            }
            println!("Data sum: {}", sum);
            
            // 释放内存
            librt::rt_safe_free(ptr);
            println!("RT-Thread memory freed");
        } else {
            println!("RT-Thread malloc failed!");
        }
    }
}

/// Comprehensive memory operations demonstration
#[no_mangle]
pub extern "C" fn rust_memory_demo_all() {
    rust_memset_demo();
    rust_memcmp_demo();
    rust_malloc_demo();
    rust_rt_malloc_demo();
}
