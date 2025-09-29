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
    unsafe {
        // 基本输出
        libc::printf(b"=== Printf Demo ===\n\0".as_ptr());
        
        // 整数格式化
        let num: i32 = 42;
        libc::printf(b"Integer: %d\n\0".as_ptr(), num);
        libc::printf(b"Hex: 0x%x\n\0".as_ptr(), num);
        libc::printf(b"Octal: %o\n\0".as_ptr(), num);
        
        // 浮点数（注意：需要软浮点支持）
        // libc::printf(b"Float: %f\n\0".as_ptr(), 3.14159);
        
        // 字符和字符串
        let ch = b'R';
        libc::printf(b"Character: %c\n\0".as_ptr(), ch as i32);
        libc::printf(b"String: %s\n\0".as_ptr(), b"RT-Thread\0".as_ptr());
        
        // 指针
        let ptr = &num as *const i32;
        libc::printf(b"Pointer: %p\n\0".as_ptr(), ptr);
    }
}

/// Using sprintf for string formatting
#[no_mangle]
pub extern "C" fn rust_sprintf_demo() -> i32 {
    let mut buffer: [u8; 128] = [0; 128];
    let result = unsafe {
        libc::sprintf(
            buffer.as_mut_ptr() as *mut c_char,
            b"Formatted: num=%d, str=%s\0".as_ptr() as *const c_char,
            100,
            b"test\0".as_ptr(),
        )
    };
    
    unsafe {
        libc::printf(b"Buffer content: %s\n\0".as_ptr(), buffer.as_ptr());
    }
    
    result
}
