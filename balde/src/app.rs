use std::env::args;
use std::ffi::{CStr, CString};
use std::ptr;

use foreign_types::{foreign_type, ForeignTypeRef};

foreign_type! {
    type CType = balde_sys::balde_app_t;
    fn drop = balde_sys::balde_app_free;

    pub struct App;
    pub struct AppRef;
}

impl App {
    pub fn new() -> Self {
        unsafe { App(balde_sys::balde_app_init()) }
    }
}

impl AppRef {
    pub fn run(&self) {
        let mut arguments: Vec<*mut i8> = args()
            .map(|arg| CString::new(arg).unwrap().into_raw())
            .collect();
        unsafe {
            balde_sys::balde_app_run(
                self.as_ptr(),
                arguments.len() as i32,
                arguments.as_mut_ptr(),
            )
        }
    }
    pub fn get_config(&self, name: &str) -> Option<&str> {
        let name = CString::new(name).unwrap();
        unsafe {
            let config = balde_sys::balde_app_get_config(self.as_ptr(), name.as_ptr());
            if config.is_null() {
                None
            } else {
                Some(CStr::from_ptr(config).to_str().unwrap())
            }
        }
    }

    pub fn set_config(&self, name: &str, value: &str) {
        let name = CString::new(name).unwrap();
        let value = CString::new(value).unwrap();
        unsafe {
            balde_sys::balde_app_set_config(self.as_ptr(), name.as_ptr(), value.as_ptr());
        }
    }

    pub fn set_config_from_env_var(&self, name: &str, env_name: &str, silent: bool) {
        let name = CString::new(name).unwrap();
        let env_name = CString::new(env_name).unwrap();

        unsafe {
            balde_sys::balde_app_set_config_from_envvar(
                self.as_ptr(),
                name.as_ptr(),
                env_name.as_ptr(),
                silent as glib_sys::gboolean,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::App;
    use std::env;
    #[test]
    fn test_config() {
        let app = App::new();
        assert_eq!(app.get_config("test"), None);
        app.set_config("test", "hi");
        assert_eq!(app.get_config("test"), Some("hi"));
    }
    #[test]
    fn test_config_from_env() {
        let app = App::new();
        assert_eq!(app.get_config("test"), None);
        env::set_var("SOME_VAR", "hi");
        app.set_config_from_env_var("test", "SOME_VAR", false);
        assert_eq!(app.get_config("test"), Some("hi"));
    }
}
