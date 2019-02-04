pub mod app;
pub mod methods;
pub mod request;
pub mod response;

pub use crate::app::{App, AppRef};
pub use crate::methods::HTTPMethod;
pub use crate::request::RequestRef;
pub use crate::response::Response;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
