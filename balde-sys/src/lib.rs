#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use gio_sys::GResource;
use glib_sys::*;
use gtypes::{gchar, gint, gssize};

type gint64 = i64;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::balde_make_response;
    use std::ffi::CString;
    #[test]
    fn smoke_test() {
        unsafe {
            let content = CString::new("OK").unwrap();
            let response = balde_make_response(content.as_ptr());
            assert_eq!((*response).status_code, 200);
        }
    }
}
