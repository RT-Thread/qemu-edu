/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2025-09-25     foxglove     test dynamic loading operations
 */

type NoArgFn = extern "C" fn();
type AddFn = extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int;
type MainFn = extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int;

/// Basic dlopen test
#[no_mangle]
pub extern "C" fn rust_dl_open_demo() {
    unsafe {
        libc::printf(b"\n=== dlopen Demo ===\n\0".as_ptr());
        
        let path = b"/hello.mo\0";
        let handle = libdl::dlopen(path.as_ptr() as *const _, libdl::RTLD_NOW | libdl::RTLD_GLOBAL);
        if handle.is_null() {
            libc::printf(b"dlopen failed: %s\n\0".as_ptr(), libdl::last_error_ptr());
        } else {
            libc::printf(b"dlopen ok: %p\n\0".as_ptr(), handle);
            libc::printf(b"Module loaded successfully\n\0".as_ptr());
            
            // Close immediately for this demo
            let rc = libdl::dlclose(handle);
            libc::printf(b"dlclose rc=%d\n\0".as_ptr(), rc);
        }
    }
}

/// dlsym symbol resolution test
#[no_mangle]
pub extern "C" fn rust_dl_sym_demo() {
    unsafe {
        libc::printf(b"\n=== dlsym Demo ===\n\0".as_ptr());
        
        let path = b"/hello.mo\0";
        let handle = libdl::dlopen(path.as_ptr() as *const _, libdl::RTLD_NOW | libdl::RTLD_GLOBAL);
        if handle.is_null() {
            libc::printf(b"dlopen failed: %s\n\0".as_ptr(), libdl::last_error_ptr());
            return;
        }
        
        // Try to resolve main symbol
        let sym_name = b"main\0";
        let sym = libdl::dlsym(handle, sym_name.as_ptr() as *const _);
        if sym.is_null() {
            libc::printf(b"dlsym main failed: %s\n\0".as_ptr(), libdl::last_error_ptr());
        } else {
            libc::printf(b"Found symbol 'main' at %p\n\0".as_ptr(), sym);
        }
        
        // Try to resolve non-existent symbol
        let bad_sym = b"nonexistent_symbol\0";
        let bad_result = libdl::dlsym(handle, bad_sym.as_ptr() as *const _);
        if bad_result.is_null() {
            libc::printf(b"dlsym nonexistent_symbol failed (expected): %s\n\0".as_ptr(), libdl::last_error_ptr());
        } else {
            libc::printf(b"Unexpected: found nonexistent symbol\n\0".as_ptr());
        }
        
        let rc = libdl::dlclose(handle);
        libc::printf(b"dlclose rc=%d\n\0".as_ptr(), rc);
    }
}

/// Function call test through dlsym
#[no_mangle]
pub extern "C" fn rust_dl_call_demo() {
    unsafe {
        libc::printf(b"\n=== Function Call Demo ===\n\0".as_ptr());
        
        let path = b"/hello.mo\0";
        let handle = libdl::dlopen(path.as_ptr() as *const _, libdl::RTLD_NOW | libdl::RTLD_GLOBAL);
        if handle.is_null() {
            libc::printf(b"dlopen failed: %s\n\0".as_ptr(), libdl::last_error_ptr());
            return;
        }
        
        // Call main function
        let sym_name = b"main\0";
        let sym = libdl::dlsym(handle, sym_name.as_ptr() as *const _);
        if sym.is_null() {
            libc::printf(b"dlsym main failed: %s\n\0".as_ptr(), libdl::last_error_ptr());
        } else {
            let main_fn: MainFn = core::mem::transmute(sym);
            libc::printf(b"Calling module main()...\n\0".as_ptr());
            let rc = main_fn(0, core::ptr::null_mut());
            libc::printf(b"module main() returned %d\n\0".as_ptr(), rc);
        }
        
        let rc = libdl::dlclose(handle);
        libc::printf(b"dlclose rc=%d\n\0".as_ptr(), rc);
    }
}

/// Error handling test
#[no_mangle]
pub extern "C" fn rust_dl_error_demo() {
    unsafe {
        libc::printf(b"\n=== Error Handling Demo ===\n\0".as_ptr());
        
        // Try to open non-existent module
        let bad_path = b"/nonexistent.mo\0";
        let handle = libdl::dlopen(bad_path.as_ptr() as *const _, libdl::RTLD_NOW);
        if handle.is_null() {
            libc::printf(b"dlopen nonexistent module failed (expected): %s\n\0".as_ptr(), libdl::last_error_ptr());
        } else {
            libc::printf(b"Unexpected: loaded nonexistent module\n\0".as_ptr());
            libdl::dlclose(handle);
        }
        
        // Try to open valid module with invalid flags
        let path = b"/hello.mo\0";
        let handle2 = libdl::dlopen(path.as_ptr() as *const _, 0xFFFF); // Invalid flags
        if handle2.is_null() {
            libc::printf(b"dlopen with invalid flags failed: %s\n\0".as_ptr(), libdl::last_error_ptr());
        } else {
            libc::printf(b"dlopen with invalid flags succeeded\n\0".as_ptr());
            libdl::dlclose(handle2);
        }
    }
}

/// Comprehensive dl operations demonstration
#[no_mangle]
pub extern "C" fn rust_dl_demo_all() {
    rust_dl_open_demo();
    rust_dl_sym_demo();
    rust_dl_call_demo();
    rust_dl_error_demo();
}


