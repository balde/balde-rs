use balde::{App, HTTPMethod, Response};

fn main() {
    let mut app = App::new();
    app.add_url_rule("home", "/", HTTPMethod::GET, |_app, request| {
        Response::new(&format!("hallo {:?}", request.get_header("Host")))
    });
    app.run();
}
