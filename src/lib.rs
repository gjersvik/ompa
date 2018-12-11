#![deny(clippy::all)]

mod housework;
mod models;
mod test_actions;
mod views;

use actix_web::{server, App, HttpRequest, HttpResponse};

fn index(_req: &HttpRequest) -> HttpResponse {
    let test = test_actions::test_actions();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(views::view_list(&test))
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn start() {
    println!("{:?}", std::env::current_exe());
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(addr)
        .unwrap()
        .run();
}