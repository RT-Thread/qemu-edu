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
type AddFn = extern "C" fn(c_int, c_int) -> c_int;
type MainFn = extern "C" fn(c_int, *mut *mut c_char) -> c_int;
// 为宏提供 `libloading` 命名空间别名
use crate::libloader::libloading;

// 使用宏生成动态库函数的便捷调用封装
// 注意：路径为字符串，不需要以 NUL 结尾
/// Basic dlopen test
#[no_mangle]
pub extern "C" fn rust_dl_open_demo() {
    unsafe {
        println!("=== dlopen Demo ===");
        
        let path = b"/libmylib.mo\0";
        match libloader::dl_open(path.as_ptr() as *const _) {
            Err(_) => {
                println!("dlopen failed:");
                libloader::dl_print_last_error();
            },
            Ok(lib) => {
                println!("Module loaded successfully");
                match lib.close() {
                    Ok(()) => println!("dlclose success"),
                    Err(e) => println!("dlclose failed: {}", e),
                }
            }
        }
    }
}

/// dlsym symbol resolution test
#[no_mangle]
pub extern "C" fn rust_dl_sym_demo() {
    unsafe {
        println!("=== dlsym Demo ===");
        
        let path = b"/hello.mo\0";
        let lib = match libloader::dl_open(path.as_ptr() as *const _) {
            Err(_) => {
                println!("dlopen failed:");
                libloader::dl_print_last_error();
                return;
            }
            Ok(lib) => lib,
        };

        // Try to resolve main symbol
        let sym_name = b"main\0";
        match libloader::dl_sym::<MainFn>(&lib, sym_name.as_ptr() as *const _) {
            Err(_) => {
                println!("dlsym main failed:");
                libloader::dl_print_last_error();
            },
            Ok(main_sym) => {
                let ptr: *const c_void = main_sym.as_raw() as *const c_void;
                println!("Found symbol 'main' at {:p}", ptr);
            }
        }

        // Try to resolve non-existent symbol
        let bad_sym = b"nonexistent_symbol\0";
        match libloader::dl_sym::<NoArgFn>(&lib, bad_sym.as_ptr() as *const _) {
            Err(_) => {
                println!("dlsym nonexistent_symbol failed (expected):");
                libloader::dl_print_last_error();
            },
            Ok(_) => println!("Unexpected: found nonexistent symbol"),
        }

        match lib.close() {
            Ok(()) => println!("dlclose success"),
            Err(e) => println!("dlclose failed: {}", e),
        }
    }
}

/// Function call test through dlsym
#[no_mangle]
pub extern "C" fn rust_dl_call_demo() {
    unsafe {
        println!("\n=== Function Call Demo ===");
        
        let path = b"/libmylib.mo\0";
        let lib = match libloader::dl_open(path.as_ptr() as *const _) {
            Err(_) => {
                println!("dlopen failed:");
                libloader::dl_print_last_error();
                return;
            }
            Ok(lib) => lib,
        };

        // Call add function
        let add_sym_name = b"rust_mylib_add\0";
        match libloader::dl_sym::<AddFn>(&lib, add_sym_name.as_ptr() as *const _) {
            Err(_) => {
                println!("dlsym add failed:");
                libloader::dl_print_last_error();
            },
            Ok(add) => {
                let ptr: *const c_void = add.as_raw() as *const c_void;
                println!("Found symbol 'add' at {:p}", ptr);
                println!("Calling add(5, 3)...");
                let f: AddFn = add.to_value();
                let result = f(5, 3);
                println!("add(5, 3) = {}", result);
            }
        }

        // Call main function (if exists)
        let sym_name = b"main\0";
        match libloader::dl_sym::<MainFn>(&lib, sym_name.as_ptr() as *const _) {
            Err(_) => {
                println!("dlsym main failed:");
                libloader::dl_print_last_error();
            },
            Ok(main) => {
                println!("Calling module main()...");
                let f: MainFn = main.to_value();
                let rc = f(0, core::ptr::null_mut());
                println!("module main() returned {}", rc);
            }
        }

        match lib.close() {
            Ok(()) => println!("dlclose success"),
            Err(e) => println!("dlclose failed: {}", e),
        }
    }
}

/// 使用 get_libfn! 宏的演示
#[no_mangle]
pub extern "C" fn rust_dl_macro_demo() {
    println!("\n=== Macro Demo ===");

    // 直接调用宏生成的函数
    get_libfn!("/hello.mo", "main", my_hello, ());
    my_hello();

    get_libfn!("/libmylib.mo", "rust_mylib_add", my_add, c_int, a: c_int, b: c_int);
    let s = my_add(15, 20);
    println!("my_add(15, 20) = {}", s);

    get_libfn!("/libmylib.mo", "rust_mylib_println", my_println, (), s: *const c_char);
    my_println(b"rustlib: Hello World\0".as_ptr() as *const c_char);
}

/// Add function call demonstration
#[no_mangle]
pub extern "C" fn rust_dl_add_demo() {
    unsafe {
        println!("\n=== Add Function Demo ===");
        
        let path = b"/libmylib.mo\0";
        let lib = match libloader::dl_open(path.as_ptr() as *const _) {
            Err(_) => {
                println!("dlopen failed:");
                libloader::dl_print_last_error();
                return;
            }
            Ok(lib) => lib,
        };

        // Resolve add function symbol
        let add_sym_name = b"rust_mylib_add\0";
        match libloader::dl_sym::<AddFn>(&lib, add_sym_name.as_ptr() as *const _) {
            Err(_) => {
                println!("dlsym add failed:");
                libloader::dl_print_last_error();
            },
            Ok(add) => {
                println!("Successfully loaded add function from dynamic module");
                let test_cases = [(10, 20), (100, 200), (-5, 15), (0, 42)];
                for (a, b) in test_cases.iter() {
                    let f: AddFn = add.to_value();
                    let result = f(*a, *b);
                    println!("add({}, {}) = {}", *a, *b, result);
                }
            }
        }

        match lib.close() {
            Ok(()) => println!("dlclose rc=0"),
            Err(e) => println!("dlclose failed: {}", e),
        }
    }
}

/// Error handling test
#[no_mangle]
pub extern "C" fn rust_dl_error_demo() {
    unsafe {
        println!("\n=== Error Handling Demo ===");
        
        // Try to open non-existent module
        let bad_path = b"/nonexistent.mo\0";
        match libloader::dl_open(bad_path.as_ptr() as *const _) {
            Err(_) => {
                println!("dlopen nonexistent module failed (expected):");
                libloader::dl_print_last_error();
            },
            Ok(lib) => {
                println!("Unexpected: loaded nonexistent module");
                let _ = lib.close();
            }
        }
        
        // Try to open valid module with invalid flags
        let path = b"/hello.mo\0";
        match libloader::dl_open_with_flags(path.as_ptr() as *const _, 0xFFFF) {
            Err(_) => {
                println!("dlopen with invalid flags failed:");
                libloader::dl_print_last_error();
            },
            Ok(lib) => {
                println!("dlopen with invalid flags succeeded");
                let _ = lib.close();
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
    rust_dl_add_demo();
    rust_dl_error_demo();
    rust_dl_macro_demo();
}
