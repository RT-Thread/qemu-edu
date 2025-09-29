/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       foxglove
 * 2025-09-15     foxglove     1.0 version
 */

//! RT-Thread OS API bindings
//! 
//! Provides Rust wrappers for RT-Thread kernel and device driver APIs

use core::ffi::{c_char, c_int, c_void, c_uint, c_ulong};

// RT-Thread basic type definitions
pub type rt_base_t = c_ulong;
pub type rt_ubase_t = c_ulong;
pub type rt_tick_t = c_uint;
pub type rt_size_t = c_ulong;
pub type rt_err_t = c_int;
pub type rt_thread_t = *mut c_void;
pub type rt_sem_t = *mut c_void;
pub type rt_mutex_t = *mut c_void;
pub type rt_device_t = *mut c_void;

// RT-Thread error codes
pub const RT_EOK: rt_err_t = 0;
pub const RT_ERROR: rt_err_t = 1;
pub const RT_ETIMEOUT: rt_err_t = 2;
pub const RT_EFULL: rt_err_t = 3;
pub const RT_EEMPTY: rt_err_t = 4;
pub const RT_ENOMEM: rt_err_t = 5;
pub const RT_ENOSYS: rt_err_t = 6;
pub const RT_EBUSY: rt_err_t = 7;
pub const RT_EIO: rt_err_t = 8;
pub const RT_EINTR: rt_err_t = 9;
pub const RT_EINVAL: rt_err_t = 10;

// ============== Kernel object management ==============
extern "C" {
    pub fn rt_object_get_type(object: *mut c_void) -> u8;
    pub fn rt_object_find(name: *const c_char, object_type: u8) -> *mut c_void;
}


// ============== Debug output ==============
extern "C" {
    pub fn rt_kprintf(fmt: *const u8, ...) -> c_int;
}