/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       foxglove
 * 2025-09-15     foxglove     1.0 version
 * 2025-09-23     foxglove     1.1 version, add libdl bindings
 */

//! C standard library API bindings
//! 
//! Provides Rust wrappers for standard C library functions

pub use core::ffi::{c_char, c_int, c_void, c_uint, c_long};

// RT-Thread libdl typically supports these flags; define minimally used ones
pub const RTLD_LAZY: c_int = 0x00000;   // Lazy function call binding
pub const RTLD_NOW: c_int = 0x00001;    // Immediate function call binding
pub const RTLD_GLOBAL: c_int = 0x10000; // Make symbols globally available
pub const RTLD_LOCAL: c_int = 0x00000;       // Default local

// ============== time functions ==============
pub type time_t = c_long;
pub type suseconds_t = c_int;
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

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

// ============== libdl functions ==============
extern "C" {
    pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> c_int;
    pub fn dlerror() -> *const c_char;
}

// ============== time functions ==============
extern "C" {
    // Use local types instead of crate-root paths to avoid E0412
    pub fn gettimeofday(tp: *mut timeval, tz: *mut c_void) -> c_int;
}



// ============== Rust-friendly wrappers ==============

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

/// Helper: get last libdl error C-string pointer
/// Safe wrapper around `dlerror()` returning the raw pointer for printing.
#[inline]
pub fn last_error_ptr() -> *const c_char {
    unsafe { dlerror() }
}