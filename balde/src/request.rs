use balde_sys;
use foreign_types::{foreign_type, ForeignTypeRef};
use std::ffi::{CStr, CString};

foreign_type! {
    type CType = balde_sys::balde_request_t;
    fn drop = |_p| {};

    pub struct Request;
    pub struct RequestRef;
}

impl RequestRef {
    pub fn get_header(&self, name: &str) -> Option<&str> {
        let name = CString::new(name).unwrap();
        let value = unsafe { balde_sys::balde_request_get_header(self.as_ptr(), name.as_ptr()) };
        if value.is_null() {
            None
        } else {
            unsafe { Some(CStr::from_ptr(value).to_str().unwrap()) }
        }
    }
}
