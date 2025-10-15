/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2025-09-20     foxglove     test printf operations
 */

// Printf examples - demonstrates formatted output functionality

/// Demonstrate various printf formats
#[no_mangle]
pub extern "C" fn rust_printf_demo() {
    // 基本输出
    println!("=== Printf Demo ===");
    
    // 整数格式化
    let num: i32 = 42;
    println!("Integer: {}", num);
    println!("Hex: 0x{:x}", num);
    println!("Octal: {:o}", num);
    
    // 浮点数（注意：需要软浮点支持）
    // println!("Float: {}", 3.14159);
    
    // 字符和字符串
    let ch = 'R';
    println!("Character: {}", ch);
    println!("String: {}", "RT-Thread");
    
    // 指针
    let ptr = &num as *const i32;
    println!("Pointer: {:p}", ptr);
}

/// Using format! for string formatting
#[no_mangle]
pub extern "C" fn rust_sprintf_demo() -> i32 {
    use alloc::format;
    
    // 使用 Rust 的 format! 宏进行字符串格式化
    let formatted_string = format!("Formatted: num={}, str={}", 100, "test");
    
    println!("Buffer content: {}", formatted_string);
    
    // 返回格式化字符串的长度
    formatted_string.len() as i32
}

#[no_mangle]
pub extern "C" fn rust_vec_demo() {
    println!("\n=== Standard Vec Demo with GlobalAllocator ===");
    
    // Create a Vec with initial capacity
    let mut v: Vec<u32> = Vec::with_capacity(2);
    println!("Created Vec with capacity: {}", v.capacity());
    
    // Push elements
    for i in 1..=5u32 {
        v.push(i);
        println!("push {} -> ok", i);
    }
    
    println!("len={} cap={}", v.len(), v.capacity());
    
    // Print all elements
    for (index, &value) in v.iter().enumerate() {
        println!("v[{}]={}", index, value);
    }
    
    // Test pop operation
    println!("\nTesting pop operations:");
    while let Some(value) = v.pop() {
        println!("popped: {}, remaining len: {}", value, v.len());
    }
    
    // Test Vec methods
    println!("\nTesting Vec methods:");
    v.extend_from_slice(&[10, 20, 30, 40, 50]);
    println!("After extend_from_slice: len={}", v.len());
    
    // Test indexing
    if let Some(&value) = v.get(2) {
        println!("v[2] = {}", value);
    }
    
    // Test clear
    v.clear();
    println!("After clear: len={}, cap={}", v.len(), v.capacity());
    
    println!("Vec demo completed!");
    
    println!("\n=== Advanced Vec Operations Demo ===");
    
    // Test Vec<String> (if we had String support)
    let mut numbers: Vec<i32> = Vec::new();
    
    // Test reserve
    numbers.reserve(10);
    println!("After reserve(10): cap={}", numbers.capacity());
    
    // Fill with data
    for i in 0..10 {
        numbers.push(i * i);
    }
    
    // Test retain
    numbers.retain(|&x| x % 2 == 0);
    println!("After retain (even numbers only): len={}", numbers.len());
    
    for (i, &num) in numbers.iter().enumerate() {
        println!("numbers[{}] = {}", i, num);
    }
    
    // Test insert and remove
    numbers.insert(0, 999);
    println!("After insert(0, 999): first element = {}", numbers[0]);
    
    let removed = numbers.remove(0);
    println!("Removed element: {}", removed);
    
    // Test shrink_to_fit
    numbers.shrink_to_fit();
    println!("After shrink_to_fit: len={}, cap={}", numbers.len(), numbers.capacity());
    
    println!("Advanced Vec demo completed!");
}