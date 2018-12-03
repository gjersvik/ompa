extern crate gotham;
extern crate hyper;
extern crate mime;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod models;
mod test_actions;
mod views;

use gotham::state::State;

/// Create a `Handler` which calls the Tera static reference, renders
/// a template with a given Context, and returns the result as a String
/// to be used as Response Body
pub fn say_hello(state: State) -> (State, (mime::Mime, String)) {
    let test = test_actions::test_actions();
    let rendered = views::view_list(&test);

    (state, (mime::TEXT_HTML, rendered))
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn start() {
    println!("{:?}", std::env::current_exe());
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}