/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2025-09-25     foxglove     test vec operations
 */
/// A minimal vector backed by C malloc/realloc/free and memcpy.
/// Only supports `T: Copy` and no drop semantics for elements.
struct CVec<T: Copy> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T: Copy> CVec<T> {
    fn new() -> Self {
        CVec { ptr: core::ptr::null_mut(), len: 0, cap: 0 }
    }

    fn with_capacity(cap: usize) -> Self {
        if cap == 0 {
            return Self::new();
        }
        let bytes = cap.saturating_mul(mem::size_of::<T>());
        let raw = unsafe { libc::malloc(bytes) } as *mut T;
        if raw.is_null() {
            CVec { ptr: core::ptr::null_mut(), len: 0, cap: 0 }
        } else {
            CVec { ptr: raw, len: 0, cap }
        }
    }

    fn len(&self) -> usize { self.len }
    fn capacity(&self) -> usize { self.cap }
    fn as_ptr(&self) -> *const T { self.ptr as *const T }
    fn as_mut_ptr(&mut self) -> *mut T { self.ptr }

    fn grow(&mut self) -> bool {
        let new_cap = if self.cap == 0 { 4 } else { self.cap.saturating_mul(2) };
        let new_bytes = new_cap.saturating_mul(mem::size_of::<T>());

        let new_ptr = if self.ptr.is_null() {
            unsafe { libc::malloc(new_bytes) as *mut T }
        } else {
            unsafe { libc::realloc(self.ptr as *mut c_void, new_bytes) as *mut T }
        };

        if new_ptr.is_null() {
            return false;
        }
        self.ptr = new_ptr;
        self.cap = new_cap;
        true
    }

    fn push(&mut self, value: T) -> bool {
        if self.len == self.cap {
            if !self.grow() { return false; }
        }
        unsafe { ptr::write(self.ptr.add(self.len), value); }
        self.len += 1;
        true
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        let value = unsafe { ptr::read(self.ptr.add(self.len)) };
        Some(value)
    }

    fn get(&self, index: usize) -> Option<T> {
        if index >= self.len { return None; }
        Some(unsafe { ptr::read(self.ptr.add(index)) })
    }
}

impl<T: Copy> Drop for CVec<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { libc::free(self.ptr as *mut c_void); }
        }
        self.ptr = core::ptr::null_mut();
        self.len = 0;
        self.cap = 0;
    }
}
#[no_mangle]
pub extern "C" fn rust_vec_demo() {
    unsafe { libc::printf(b"\n=== CVec Demo ===\n\0".as_ptr()); }
    let mut v: CVec<u32> = CVec::with_capacity(2);
    for i in 1..=5u32 {
        let ok = v.push(i);
        unsafe { libc::printf(b"push %u -> %s\n\0".as_ptr(), i, if ok { b"ok\0".as_ptr() } else { b"fail\0".as_ptr() }); }
    }
    unsafe { libc::printf(b"len=%zu cap=%zu\n\0".as_ptr(), v.len(), v.capacity()); }
    for i in 0..v.len() {
        let val = v.get(i).unwrap_or(0);
        unsafe { libc::printf(b"v[%zu]=%u\n\0".as_ptr(), i, val); }
    }
}
