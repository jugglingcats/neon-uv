#![allow(non_camel_case_types)]

#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

use std::os::raw::c_void;
use std::os::raw::c_int;
use std::ptr::null_mut;

use std::thread;

use std::boxed::Box;

#[repr(C)]
pub struct uv_async_t {
    data: *mut c_void,
    _private_fields: [u8; 216]
}

impl uv_async_t {
    pub fn new() -> uv_async_t {
        uv_async_t { data: null_mut(), _private_fields: [0; 216] }
    }
}

pub type uv_async_cb = extern "C" fn(*mut uv_async_t);

extern {
    pub fn uv_default_loop() -> *mut c_void;
    pub fn uv_async_init(loop_: *mut c_void, async: *mut uv_async_t, async_cb: uv_async_cb) -> c_int;
    pub fn uv_async_send(async: *mut uv_async_t) -> c_int;
    pub fn uv_close(async: *mut uv_async_t, close_cb: *mut c_void);
    //    pub fn uv_queue_work(_loop: *mut c_void, req: *mut uv_work_t) -> c_int;
}

extern "C" fn callback(handle: *mut uv_async_t) {
    unsafe {
        let param = Box::from_raw((*handle).data as *mut i32);
        println!("Called from libuv!! Param={:?}", param);
        uv_close(handle, null_mut());
    }
}

fn hello(call: Call) -> JsResult<JsString> {
    let closure = || {
        unsafe {
            let handle = Box::new(uv_async_t::new());
            let handle = Box::into_raw(handle) as *mut uv_async_t; // malloc(size_of::<uv_async_t>()) as *mut uv_async_t;

            let param = Box::new(42);
            let param = Box::into_raw(param) as *mut c_void;

            (*handle).data = param;

            uv_async_init(uv_default_loop(), handle, callback);
            uv_async_send(handle);
        }
    };
    let _ = thread::spawn(closure).join();

    let scope = call.scope;
    Ok(JsString::new(scope, "Return value from addon").unwrap())
}

register_module!(m, {
    m.export("hello", hello)
});

