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

#![allow(non_camel_case_types)]

pub use core::ffi::{c_char, c_int, c_void, c_uint, c_long};

// RT-Thread libdl typically supports these flags; define minimally used ones
pub const RTLD_LAZY: c_int = 0x00000;   // Lazy function call binding
pub const RTLD_NOW: c_int = 0x00001;    // Immediate function call binding
pub const RTLD_GLOBAL: c_int = 0x10000; // Make symbols globally available
pub const RTLD_LOCAL: c_int = 0x00000;       // Default local

// ============== time functions ==============
pub type time_t = c_long;
pub type suseconds_t = c_int;
#[repr(C)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

// ============== libdl functions ==============
unsafe extern "C" {
    pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> c_int;
    pub fn dlerror() -> *const c_char;
}

// ============== time functions ==============
unsafe extern "C" {
    // Use local types instead of crate-root paths to avoid E0412
    pub fn gettimeofday(tp: *mut timeval, tz: *mut c_void) -> c_int;
}

/// Helper: get last libdl error C-string pointer
/// Safe wrapper around `dlerror()` returning the raw pointer for printing.
#[inline]
pub fn last_error_ptr() -> *const c_char {
    unsafe { dlerror() }
}