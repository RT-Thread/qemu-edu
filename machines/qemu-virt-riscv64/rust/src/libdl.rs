/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2025-09-23     foxglove    libdl bindings
 */

//! POSIX libdl API bindings for RT-Thread
//! Provides Rust FFI for dlopen/dlsym/dlclose/dlerror.

use core::ffi::{c_char, c_int, c_void};

// RT-Thread libdl typically supports these flags; define minimally used ones
pub const RTLD_LAZY: c_int = 0x0001;   // Lazy function call binding
pub const RTLD_NOW: c_int = 0x0002;    // Immediate function call binding
pub const RTLD_GLOBAL: c_int = 0x0100; // Make symbols globally available
pub const RTLD_LOCAL: c_int = 0;       // Default local

extern "C" {
    pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> c_int;
    pub fn dlerror() -> *const c_char;
}

/// Return last error as pointer (C string). Caller ensures non-null before use.
pub fn last_error_ptr() -> *const c_char {
    unsafe { dlerror() }
}


