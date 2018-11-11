extern crate tdjson_sys;

use tdjson_sys::*;

use std::os::raw::{
    c_void,
    c_char,
};

use std::ffi::{
    CString,
    IntoStringError,
};

use std::time::Duration;
use std::ops::Drop;

pub struct Client {
    client_ptr: *mut c_void
}

impl Client {
    pub fn new() -> Self {
        unsafe {
            Client {
                client_ptr: td_json_client_create()
            }
        }
    }

    pub fn execute(&mut self, request: &str) -> Result<String, IntoStringError> {
        let request = CString::new(request).unwrap();
        unsafe {
            let answer = td_json_client_execute(
                self.client_ptr,
                request.as_ptr() as *const c_char
            );

            let answer = answer as *mut c_char;

            CString::from_raw(answer).into_string()
        }
    }

    pub fn send(&mut self, request: &str) {
        let request = CString::new(request).unwrap();
        unsafe {
            td_json_client_send(
                self.client_ptr,
                request.as_ptr() as *const c_char
            )
        }
    }

    pub fn receive(&mut self, timeout: Duration) -> Option<Result<String, IntoStringError>> {
        let timeout = timeout.as_secs() as f64;

        unsafe {
            let answer = td_json_client_receive(
                self.client_ptr,
                timeout
            );

            let answer = answer as *mut c_char;
            if answer.is_null() {
                return None;
            }
            Some(CString::from_raw(answer).into_string())
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            td_json_client_destroy(self.client_ptr)
        }
    }
}
