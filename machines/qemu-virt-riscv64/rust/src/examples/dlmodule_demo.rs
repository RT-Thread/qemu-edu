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

/// Test loading and calling functions from libdlmodule.so
#[no_mangle]
pub extern "C" fn rust_dl_libdlmodule_demo() {
    unsafe {
        libc::printf(b"\n=== libdlmodule.so Test Demo ===\n\0".as_ptr());
        
        let path = b"/libdlmodule.so\0";
        
        // Try to load the library
        libc::printf(b"Attempting to load library...\n\0".as_ptr());
        
        match libdl::DlHandle::open(path, libdl::RTLD_NOW | libdl::RTLD_GLOBAL) {
            Ok(handle) => {
                libc::printf(b"Successfully loaded libdlmodule.so\n\0".as_ptr());
                
                // Try to resolve the 'add' function
                let add_sym_name = b"add\0";
                match handle.get_symbol(add_sym_name) {
                    Ok(sym) => {
                        libc::printf(b"Found 'add' function at %p\n\0".as_ptr(), sym);
                        
                        // Cast the symbol to the correct function type
                        let add_fn: AddFn = core::mem::transmute(sym);
                        
                        // Test the add function with different values
                        let test_cases = [(5, 3), (10, 20), (-5, 15), (0, 0), (100, -50)];
                        
                        libc::printf(b"Testing add function:\n\0".as_ptr());
                        for (a, b) in test_cases.iter() {
                            let result = add_fn(*a, *b);
                            libc::printf(b"  add(%d, %d) = %d\n\0".as_ptr(), *a, *b, result);
                        }
                        
                        // Verify correctness
                        let verification_result = add_fn(42, 58);
                        if verification_result == 100 {
                            libc::printf(b"[OK] Add function works correctly! (42 + 58 = %d)\n\0".as_ptr(), verification_result);
                        } else {
                            libc::printf(b"[FAIL] Add function failed verification! Expected 100, got %d\n\0".as_ptr(), verification_result);
                        }
                        
                    }
                    Err(error) => {
                        libc::printf(b"Failed to find 'add' function: \0".as_ptr());
                        libc::print_str(&error);
                        libc::printf(b"\n\0".as_ptr());
                    }
                }
                
                // Handle will be automatically closed when it goes out of scope
            }
            Err(error) => {
                libc::printf(b"Failed to load libdlmodule.so: \0".as_ptr());
                libc::print_str(&error);
                libc::printf(b"\n\0".as_ptr());
            }
        }
    }
}

/// Test multiple operations on libdlmodule.so
#[no_mangle]
pub extern "C" fn rust_dl_libdlmodule_stress_test() {
    unsafe {
        libc::printf(b"\n=== libdlmodule.so Stress Test ===\n\0".as_ptr());
        
        let path = b"/libdlmodule.so\0";
        
        // Test multiple load/unload cycles
        for i in 1..=3 {
            libc::printf(b"Load cycle %d:\n\0".as_ptr(), i);
            
            match libdl::DlHandle::open(path, libdl::RTLD_NOW) {
                Ok(handle) => {
                    match handle.get_symbol(b"add\0") {
                        Ok(sym) => {
                            let add_fn: AddFn = core::mem::transmute(sym);
                            let result = add_fn(i * 10, i * 5);
                            libc::printf(b"  Cycle %d: add(%d, %d) = %d\n\0".as_ptr(), i, i * 10, i * 5, result);
                            
                            if result == i * 15 {
                                libc::printf(b"[OK] Stress test iteration %d: Add function works correctly! (%d + %d = %d)\n\0".as_ptr(), i, i * 10, i * 5, result);
                            } else {
                                libc::printf(b"[FAIL] Stress test iteration %d: Add function failed! Expected %d, got %d\n\0".as_ptr(), i, i * 15, result);
                            }
                        }
                        Err(error) => {
                            libc::printf(b"  Failed to get symbol in cycle %d: \0".as_ptr(), i);
                            libc::print_str(&error);
                            libc::printf(b"\n\0".as_ptr());
                        }
                    }
                    // Handle automatically closed here
                }
                Err(error) => {
                    libc::printf(b"  Failed to load in cycle %d: \0".as_ptr(), i);
                    libc::print_str(&error);
                    libc::printf(b"\n\0".as_ptr());
                }
            }
        }
        
        libc::printf(b"Stress test completed\n\0".as_ptr());
    }
}

/// Test different possible paths for libdlmodule.so
#[no_mangle]
pub extern "C" fn rust_dl_path_test() {
    unsafe {
        libc::printf(b"\n=== Path Test for libdlmodule.so ===\n\0".as_ptr());
        
        let paths_to_try: &[&[u8]] = &[
            b"/libdlmodule.so\0",
            b"./libdlmodule.so\0",
            b"/usr/lib/libdlmodule.so\0",
            b"/lib/libdlmodule.so\0",
            b"/tmp/libdlmodule.so\0",
            b"libdlmodule.so\0",
        ];
        
        for path in paths_to_try.iter() {
            libc::printf(b"\nTesting path: %s\n\0".as_ptr(), path.as_ptr());
            
            // Try to load it directly
            match libdl::DlHandle::open(path, libdl::RTLD_NOW) {
                Ok(_handle) => {
                    libc::printf(b"  Successfully loaded!\n\0".as_ptr());
                    return; // Found working path
                }
                Err(error) => {
                    libc::printf(b"  Failed to load: \0".as_ptr());
                    libc::print_str(&error);
                    libc::printf(b"\n\0".as_ptr());
                }
            }
        }
        
        libc::printf(b"\nNo working path found for libdlmodule.so\n\0".as_ptr());
    }
}

/// Comprehensive dl operations demonstration
#[no_mangle]
pub extern "C" fn rust_dl_demo_all() {
    // rust_dl_open_demo();
    // rust_dl_sym_demo();
    // rust_dl_call_demo();
    // rust_dl_error_demo();
    rust_dl_path_test();
    rust_dl_libdlmodule_demo();
    rust_dl_libdlmodule_stress_test();
}


