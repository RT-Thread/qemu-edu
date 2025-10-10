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
