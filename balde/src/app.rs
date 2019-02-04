use std::env::args;
use std::ffi::{c_void, CStr, CString};

use std::collections::HashMap;

use lazy_static::lazy_static;

use glib_sys::gpointer;

use crate::methods::HTTPMethod;
use crate::request::RequestRef;
use crate::response::Response;

use foreign_types::{foreign_type, ForeignTypeRef};

type UserView = fn(&AppRef, &RequestRef) -> Response;

fn create_user_data() -> *mut HashMap<String, UserView> {
    let callbacks = Box::new(HashMap::new());

    Box::into_raw(callbacks)
}

unsafe extern "C" fn free_user_data(user_data: *mut c_void) {
    Box::from_raw(user_data as *mut HashMap<String, UserView>);
}

unsafe extern "C" fn request_callback(
    app: *mut balde_sys::balde_app_t,
    request: *mut balde_sys::balde_request_t,
) -> *mut balde_sys::balde_response_t {
    let (app, request) = (AppRef::from_ptr(app), RequestRef::from_ptr(request));
    let user_callbacks = app.get_user_data();
    let path = CStr::from_ptr((*request.as_ptr()).path);

    let rv = if let Some(f) = user_callbacks.get(path.to_str().unwrap()) {
        let response = f(app, request);
        response.as_ptr()
    } else {
        balde_sys::balde_abort_set_error(app.as_ptr(), 404);
        std::ptr::null_mut()
    };
    Box::into_raw(user_callbacks);
    rv
}

foreign_type! {
    type CType = balde_sys::balde_app_t;
    fn drop = balde_sys::balde_app_free;

    pub struct App;
    pub struct AppRef;
}

impl App {
    pub fn new() -> Self {
        unsafe {
            let app = App(balde_sys::balde_app_init());
            app.set_user_data();
            app
        }
    }
}

impl AppRef {
    unsafe fn set_user_data(&self) {
        balde_sys::balde_app_set_user_data(self.as_ptr(), create_user_data() as *mut c_void);
        balde_sys::balde_app_set_user_data_destroy_func(self.as_ptr(), Some(free_user_data));
    }

    pub(crate) unsafe fn get_user_data(&self) -> Box<HashMap<String, UserView>> {
        // After using, always leak the pointer to avoid free

        Box::from_raw(
            balde_sys::balde_app_get_user_data(self.as_ptr()) as *mut HashMap<String, UserView>
        )
    }

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
        let config = unsafe { balde_sys::balde_app_get_config(self.as_ptr(), name.as_ptr()) };
        if config.is_null() {
            None
        } else {
            unsafe { Some(CStr::from_ptr(config).to_str().unwrap()) }
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

    pub fn add_url_rule(&mut self, endpoint: &str, rule: &str, method: HTTPMethod, f: UserView) {
        // Register the association between url and view
        unsafe {
            let mut user_data = self.get_user_data();
            user_data.insert(rule.to_owned(), f);
            Box::into_raw(user_data); // Leak user data again otherwise we will get a segfault
        }

        let endpoint = CString::new(endpoint).unwrap();
        let rule = CString::new(rule).unwrap();

        unsafe {
            balde_sys::balde_app_add_url_rule(
                self.as_ptr(),
                endpoint.as_ptr(),
                rule.as_ptr(),
                method.bits(),
                Some(request_callback),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{App, HTTPMethod, Response};
    use std::env;
    use std::thread;
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

    #[test]
    fn test_http_call() {
        // let handler = thread::spawn(move || {
        //     let mut app = App::new();
        //     app.add_url_rule("home", "/", HTTPMethod::GET, |_, _| Response::new("hello"));
        //     app.run();
        // });
    }
}
