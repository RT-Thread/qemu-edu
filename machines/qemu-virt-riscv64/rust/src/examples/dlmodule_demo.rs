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

/// Basic dlopen test using safe wrapper
#[no_mangle]
pub extern "C" fn rust_dl_open_demo() {
    unsafe {
        libc::printf(b"\n=== dlopen Demo ===\n\0".as_ptr());
        
        let path = b"/hello.mo\0";
        match libdl::DlHandle::open(path, libdl::RTLD_NOW | libdl::RTLD_GLOBAL) {
            Ok(handle) => {
                libc::printf(b"dlopen ok: %p\n\0".as_ptr(), handle.raw_handle());
                libc::printf(b"Module loaded successfully\n\0".as_ptr());
                // Handle will be automatically closed when it goes out of scope
            }
            Err(error) => {
                libc::printf(b"dlopen failed: \0".as_ptr());
                libc::print_str(&error);
                libc::printf(b"\n\0".as_ptr());
            }
        }
    }
}

/// dlsym symbol resolution test using safe wrapper
#[no_mangle]
pub extern "C" fn rust_dl_sym_demo() {
    unsafe {
        libc::printf(b"\n=== dlsym Demo ===\n\0".as_ptr());
        
        let path = b"/hello.mo\0";
        match libdl::DlHandle::open(path, libdl::RTLD_NOW | libdl::RTLD_GLOBAL) {
            Ok(handle) => {
                // Try to resolve main symbol
                let sym_name = b"main\0";
                match handle.get_symbol(sym_name) {
                    Ok(sym) => {
                        libc::printf(b"Found symbol 'main' at %p\n\0".as_ptr(), sym);
                    }
                    Err(error) => {
                        libc::printf(b"dlsym main failed: \0".as_ptr());
                        libc::print_str(&error);
                        libc::printf(b"\n\0".as_ptr());
                    }
                }
                
                // Try to resolve non-existent symbol
                let bad_sym = b"nonexistent_symbol\0";
                match handle.get_symbol(bad_sym) {
                    Ok(_) => {
                        libc::printf(b"Unexpected: found nonexistent symbol\n\0".as_ptr());
                    }
                    Err(error) => {
                        libc::printf(b"dlsym nonexistent_symbol failed (expected): \0".as_ptr());
                        libc::print_str(&error);
                        libc::printf(b"\n\0".as_ptr());
                    }
                }
                // Handle will be automatically closed when it goes out of scope
            }
            Err(error) => {
                libc::printf(b"dlopen failed: \0".as_ptr());
                libc::print_str(&error);
                libc::printf(b"\n\0".as_ptr());
            }
        }
    }
}

/// Function call test through dlsym using safe wrapper
#[no_mangle]
pub extern "C" fn rust_dl_call_demo() {
    unsafe {
        libc::printf(b"\n=== Function Call Demo ===\n\0".as_ptr());
        
        let path = b"/hello.mo\0";
        match libdl::DlHandle::open(path, libdl::RTLD_NOW | libdl::RTLD_GLOBAL) {
            Ok(handle) => {
                // Call main function - simplified with convenience method
                let sym_name = b"main\0";
                match handle.try_call_main(sym_name) {
                    Ok(main_fn) => {
                        libc::printf(b"Calling module main()...\n\0".as_ptr());
                        let rc = main_fn(0, core::ptr::null_mut());
                        libc::printf(b"module main() returned %d\n\0".as_ptr(), rc);
                    }
                    Err(error) => {
                        libc::printf(b"dlsym main failed: \0".as_ptr());
                        libc::print_str(&error);
                        libc::printf(b"\n\0".as_ptr());
                    }
                }
                // Handle will be automatically closed when it goes out of scope
            }
            Err(error) => {
                libc::printf(b"dlopen failed: \0".as_ptr());
                libc::print_str(&error);
                libc::printf(b"\n\0".as_ptr());
            }
        }
    }
}

/// Error handling test using safe wrapper
#[no_mangle]
pub extern "C" fn rust_dl_error_demo() {
    unsafe {
        libc::printf(b"\n=== Error Handling Demo ===\n\0".as_ptr());
        
        // Try to open non-existent module
        let bad_path = b"/nonexistent.mo\0";
        match libdl::DlHandle::open(bad_path, libdl::RTLD_NOW) {
            Ok(_handle) => {
                libc::printf(b"Unexpected: loaded nonexistent module\n\0".as_ptr());
                // Handle will be automatically closed when it goes out of scope
            }
            Err(error) => {
                libc::printf(b"dlopen nonexistent module failed (expected): \0".as_ptr());
                libc::print_str(&error);
                libc::printf(b"\n\0".as_ptr());
            }
        }
        
        // Try to open valid module with invalid flags
        let path = b"/hello.mo\0";
        match libdl::DlHandle::open(path, 0xFFFF) { // Invalid flags
            Ok(_handle) => {
                libc::printf(b"dlopen with invalid flags succeeded\n\0".as_ptr());
                // Handle will be automatically closed when it goes out of scope
            }
            Err(error) => {
                libc::printf(b"dlopen with invalid flags failed: \0".as_ptr());
                libc::print_str(&error);
                libc::printf(b"\n\0".as_ptr());
            }
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


