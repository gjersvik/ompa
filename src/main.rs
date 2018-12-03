//! An example usage of Tera template engine working with Gotham.

extern crate gotham;
extern crate hyper;
extern crate mime;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tera;

use gotham::state::State;
use tera::{Context, Tera};

/// Assuming the Rust file is at the same level as the templates folder
/// we can get a Tera instance that way:
lazy_static! {
    pub static ref TERA: Tera =
        compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
}

/// Create a `Handler` which calls the Tera static reference, renders
/// a template with a given Context, and returns the result as a String
/// to be used as Response Body
pub fn say_hello(state: State) -> (State, (mime::Mime, String)) {
    let mut context = Context::new();
    context.insert("user", "Gotham");
    let rendered = TERA.render("example.html", &context).unwrap();

    (state, (mime::TEXT_HTML, rendered))
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    println!("{:?}", std::env::current_exe());
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}
