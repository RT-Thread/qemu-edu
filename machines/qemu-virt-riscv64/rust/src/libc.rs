/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2024-09-20     RT-Thread    First version
 */

//! C standard library API bindings
//! 
//! Provides Rust wrappers for standard C library functions

pub use core::ffi::{c_char, c_int, c_void, c_uint, c_long};

// ============== stdio functions ==============
extern "C" {
    pub fn printf(fmt: *const u8, ...) -> c_int;
    pub fn sprintf(str: *mut c_char, fmt: *const c_char, ...) -> c_int;
    pub fn snprintf(str: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn putchar(c: c_int) -> c_int;
}

// ============== string functions ==============
extern "C" {
    pub fn strlen(s: *const c_char) -> usize;
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    pub fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    pub fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncat(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    pub fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    pub fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

// ============== memory functions ==============
extern "C" {
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    pub fn memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void;
}

// ============== dynamic memory management ==============
extern "C" {
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    pub fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
}

// ============== math functions ==============
extern "C" {
    pub fn abs(n: c_int) -> c_int;
    pub fn labs(n: c_long) -> c_long;
    pub fn rand() -> c_int;
    pub fn srand(seed: c_uint);
}

// ============== conversion functions ==============
extern "C" {
    pub fn atoi(nptr: *const c_char) -> c_int;
    pub fn atol(nptr: *const c_char) -> c_long;
    pub fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
}

// ============== Rust-friendly wrappers ==============

/// Safe string length calculation
pub fn safe_strlen(s: *const c_char) -> usize {
    if s.is_null() {
        0
    } else {
        unsafe { strlen(s) }
    }
}

/// Safe memory allocation
pub fn safe_malloc(size: usize) -> Option<*mut c_void> {
    if size == 0 {
        None
    } else {
        let ptr = unsafe { malloc(size) };
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }
}

/// Safe memory deallocation
pub fn safe_free(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe { free(ptr) }
    }
}

/// Print string (Rust style)
pub fn print_str(s: &str) {
    for byte in s.bytes() {
        unsafe { putchar(byte as c_int) };
    }
}

/// Print null-terminated string
pub fn print_cstr(s: &[u8]) {
    unsafe { printf(s.as_ptr()) };
}