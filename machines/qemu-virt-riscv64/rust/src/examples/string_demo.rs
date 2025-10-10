/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2025-09-20     foxglove     test string operations
 */

// String operation examples - demonstrates string-related function usage

/// String length calculation example
#[no_mangle]
pub extern "C" fn rust_strlen_demo(s: *const c_char) -> usize {
    libc::safe_strlen(s)
}

/// String comparison example
#[no_mangle]
pub extern "C" fn rust_strcmp_demo(s1: *const c_char, s2: *const c_char) -> i32 {
    if s1.is_null() || s2.is_null() {
        return -1;
    }
    
    unsafe { libc::strcmp(s1, s2) }
}

/// String copy example
#[no_mangle]
pub extern "C" fn rust_strcpy_demo() {
    let mut dest: [u8; 64] = [0; 64];
    let src = b"Hello, RT-Thread!\0";
    
    unsafe {
        libc::strcpy(dest.as_mut_ptr() as *mut c_char, src.as_ptr() as *const c_char);
        let dest_str = core::ffi::CStr::from_ptr(dest.as_ptr() as *const c_char);
        if let Ok(s) = dest_str.to_str() {
            println!("Copied string: {}", s);
        } else {
            println!("Copied string: [invalid UTF-8]");
        }
    }
}

/// String concatenation example
#[no_mangle]
pub extern "C" fn rust_strcat_demo() {
    let mut buffer: [u8; 128] = [0; 128];
    
    // 初始化第一部分
    let part1 = b"Hello, \0";
    unsafe {
        libc::strcpy(buffer.as_mut_ptr() as *mut c_char, part1.as_ptr() as *const c_char);
    }
    
    // 连接第二部分
    let part2 = b"RT-Thread!\0";
    unsafe {
        libc::strcat(buffer.as_mut_ptr() as *mut c_char, part2.as_ptr() as *const c_char);
        let buffer_str = core::ffi::CStr::from_ptr(buffer.as_ptr() as *const c_char);
        if let Ok(s) = buffer_str.to_str() {
            println!("Concatenated string: {}", s);
        } else {
            println!("Concatenated string: [invalid UTF-8]");
        }
    }
}

/// String search example
#[no_mangle]
pub extern "C" fn rust_strstr_demo(haystack: *const c_char, needle: *const c_char) -> bool {
    if haystack.is_null() || needle.is_null() {
        return false;
    }
    
    let result = unsafe { libc::strstr(haystack, needle) };
    !result.is_null()
}

/// Comprehensive string operations demonstration
#[no_mangle]
pub extern "C" fn rust_string_demo_all() {
    println!("\n=== String Operations Demo ===");
    
    unsafe {
        // 长度计算
        let test_str = b"RT-Thread\0";
        let len = libc::strlen(test_str.as_ptr() as *const c_char);
        let test_str_rust = core::ffi::CStr::from_ptr(test_str.as_ptr() as *const c_char);
        if let Ok(s) = test_str_rust.to_str() {
            println!("Length of '{}': {}", s, len);
        } else {
            println!("Length of [invalid UTF-8]: {}", len);
        }
        
        // 字符串比较
        let str1 = b"abc\0";
        let str2 = b"abd\0";
        let cmp = libc::strcmp(str1.as_ptr() as *const c_char, str2.as_ptr() as *const c_char);
        let str1_rust = core::ffi::CStr::from_ptr(str1.as_ptr() as *const c_char);
        let str2_rust = core::ffi::CStr::from_ptr(str2.as_ptr() as *const c_char);
        if let (Ok(s1), Ok(s2)) = (str1_rust.to_str(), str2_rust.to_str()) {
            println!("strcmp('{}', '{}') = {}", s1, s2, cmp);
        } else {
            println!("strcmp([invalid UTF-8], [invalid UTF-8]) = {}", cmp);
        }
        
        // 字符查找
        let ch = b'd' as i32;
        let found = libc::strchr(str2.as_ptr() as *const c_char, ch);
        if !found.is_null() {
            if let Ok(s2) = str2_rust.to_str() {
                println!("Found '{}' in '{}'", ch as u8 as char, s2);
            } else {
                println!("Found '{}' in [invalid UTF-8]", ch as u8 as char);
            }
        }
    }
}
