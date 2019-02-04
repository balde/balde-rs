use balde_sys;
use foreign_types::{foreign_type, ForeignTypeRef};
use std::ffi::{CStr, CString};

foreign_type! {
    type CType = balde_sys::balde_response_t;
    fn drop = |_p| {};

    pub struct Response;
    pub struct ResponseRef;
}

impl Response {
    pub fn new(content: &str) -> Self {
        unsafe {
            let content = CString::new(content).unwrap();
            Response(balde_sys::balde_make_response(content.as_ptr()))
        }
    }
}

impl ResponseRef {}
