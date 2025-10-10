use crate::api::RttCResult;
use core::ffi::c_void;
use crate::bindings::*;
use core::ptr;
use alloc::ffi::CString;

pub type APIRawQueue = rt_mq_t;

#[inline]
pub(crate) fn queue_create(name: &str, len: u32, message_size: u32) -> Option<APIRawQueue> {
    let s = CString::new(name).unwrap();
    let raw;
    unsafe { raw = rt_mq_create(s.as_ptr(), message_size.into(), len.into(), 0) }
    if raw == ptr::null_mut() {
        None
    } else {
        Some(raw)
    }
}

#[inline]
pub(crate) fn queue_send_wait(
    handle: APIRawQueue,
    msg: *const c_void,
    msg_size: u32,
    tick: i32,
) -> RttCResult {
    unsafe { rt_mq_send_wait(handle, msg, msg_size.into(), tick).into() }
}

#[inline]
pub(crate) fn queue_receive_wait(
    handle: APIRawQueue,
    msg: *mut c_void,
    msg_size: u32,
    tick: i32,
) -> RttCResult {
    unsafe { rt_mq_recv(handle, msg, msg_size.into(), tick).into() }
}

#[inline]
pub(crate) fn queue_delete(handle: APIRawQueue) {
    unsafe {
        rt_mq_delete(handle);
    }
}