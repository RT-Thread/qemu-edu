/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2024-09-20     foxglove     test string operations
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
        libc::printf(b"Copied string: %s\n\0".as_ptr(), dest.as_ptr());
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
        libc::printf(b"Concatenated string: %s\n\0".as_ptr(), buffer.as_ptr());
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
    unsafe {
        libc::printf(b"\n=== String Operations Demo ===\n\0".as_ptr());
        
        // 长度计算
        let test_str = b"RT-Thread\0";
        let len = libc::strlen(test_str.as_ptr() as *const c_char);
        libc::printf(b"Length of '%s': %zu\n\0".as_ptr(), test_str.as_ptr(), len);
        
        // 字符串比较
        let str1 = b"abc\0";
        let str2 = b"abd\0";
        let cmp = libc::strcmp(str1.as_ptr() as *const c_char, str2.as_ptr() as *const c_char);
        libc::printf(b"strcmp('%s', '%s') = %d\n\0".as_ptr(), 
                    str1.as_ptr(), str2.as_ptr(), cmp);
        
        // 字符查找
        let ch = b'd' as i32;
        let found = libc::strchr(str2.as_ptr() as *const c_char, ch);
        if !found.is_null() {
            libc::printf(b"Found '%c' in '%s'\n\0".as_ptr(), ch, str2.as_ptr());
        }
    }
}
