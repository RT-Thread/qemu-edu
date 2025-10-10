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

#[cfg(feature = "rt_using_dlopen")]
extern "C" {
    pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> c_int;
    pub fn dlerror() -> *const c_char;
}

// Stub implementations when dynamic library support is not available
#[cfg(not(feature = "rt_using_dlopen"))]
pub unsafe fn dlopen(_filename: *const c_char, _flag: c_int) -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(not(feature = "rt_using_dlopen"))]
pub unsafe fn dlsym(_handle: *mut c_void, _symbol: *const c_char) -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(not(feature = "rt_using_dlopen"))]
pub unsafe fn dlclose(_handle: *mut c_void) -> c_int {
    -1
}

#[cfg(not(feature = "rt_using_dlopen"))]
pub unsafe fn dlerror() -> *const c_char {
    b"Dynamic library support not available\0".as_ptr() as *const c_char
}

/// Return last error as pointer (C string). Caller ensures non-null before use.
pub fn last_error_ptr() -> *const c_char {
    unsafe { dlerror() }
}

// ============== Rust-friendly safe wrappers ==============

/// Dynamic library handle wrapper for safe resource management
pub struct DlHandle {
    handle: *mut c_void,
}

impl DlHandle {
    /// Open a dynamic library safely
    pub fn open(filename: &[u8], flags: c_int) -> Result<Self, &'static str> {
        // Use the safe_dlopen function for consistency
        safe_dlopen(filename, flags)
    }
    
    /// Get symbol from the dynamic library safely
    pub fn get_symbol(&self, symbol: &[u8]) -> Result<*mut c_void, &'static str> {
        // Use the safe_dlsym function for consistency
        safe_dlsym(self.handle, symbol)
    }
    
    /// Get a function pointer from the dynamic library
    /// This is a convenience method that combines get_symbol with transmute
    pub fn get_function<F>(&self, symbol: &[u8]) -> Result<F, &'static str> {
        let sym = self.get_symbol(symbol)?;
        Ok(unsafe { core::mem::transmute_copy(&sym) })
    }
    
    /// Try to call a function with no arguments
    /// Returns the function pointer if found, None if not found
    pub fn try_call_no_args(&self, symbol: &[u8]) -> Result<extern "C" fn(), &'static str> {
        self.get_function(symbol)
    }
    
    /// Try to call a main-style function
    /// Returns the function pointer if found
    pub fn try_call_main(&self, symbol: &[u8]) -> Result<extern "C" fn(c_int, *mut *mut c_char) -> c_int, &'static str> {
        self.get_function(symbol)
    }
    
    /// Get the raw handle (for compatibility with existing code)
    pub fn raw_handle(&self) -> *mut c_void {
        self.handle
    }
}

impl Drop for DlHandle {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            // Use safe_dlclose for consistency, but ignore the result in Drop
            let _ = safe_dlclose(self.handle);
        }
    }
}

/// Safe dlopen wrapper
pub fn safe_dlopen(filename: &[u8], flags: c_int) -> Result<DlHandle, &'static str> {
    let handle = unsafe { dlopen(filename.as_ptr() as *const c_char, flags) };
    
    if handle.is_null() {
        // For embedded environment, use static error message
        Err("Failed to open dynamic library")
    } else {
        Ok(DlHandle { handle })
    }
}

/// Safe dlsym wrapper (requires valid handle)
pub fn safe_dlsym(handle: *mut c_void, symbol: &[u8]) -> Result<*mut c_void, &'static str> {
    if handle.is_null() {
        return Err("Invalid handle");
    }
    
    let sym = unsafe { dlsym(handle, symbol.as_ptr() as *const c_char) };
    
    if sym.is_null() {
        // For embedded environment, use static error message
        Err("Failed to find symbol in dynamic library")
    } else {
        Ok(sym)
    }
}

/// Safe dlclose wrapper
pub fn safe_dlclose(handle: *mut c_void) -> Result<(), c_int> {
    if handle.is_null() {
        return Err(-1);
    }
    
    let result = unsafe { dlclose(handle) };
    if result == 0 {
        Ok(())
    } else {
        Err(result)
    }
}

/// Safe dlerror wrapper - returns error message as static string
pub fn safe_dlerror() -> Option<&'static str> {
    let error_ptr = unsafe { dlerror() };
    if error_ptr.is_null() {
        None
    } else {
        // For embedded environment, return generic error message
        Some("Dynamic library error occurred")
    }
}
