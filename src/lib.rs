#![deny(clippy::all)]

mod housework;
mod models;
mod test_actions;
mod views;

use actix_web::error::ErrorInternalServerError;
use actix_web::AsyncResponder;
use actix_web::Error;
use futures::future::Future;
use actix_web::{server, App, HttpRequest, HttpResponse};
use actix::{Arbiter, System};

use crate::{
    models::GetActions,
    test_actions::TestActions,
    housework::Chores,
};

fn index(_req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let test = Arbiter::registry().get::<TestActions>();
    let chore = System::current().registry().get::<Chores>();

    let res = test.send(GetActions{}).join(chore.send(GetActions{}));

    res.map(|(a,b)|{
        let mut a = a.actions;
        let mut b = b.actions;
        a.append(&mut b);
        a
    }).map(|actions| {
        HttpResponse::Ok()
        .content_type("text/html")
        .body(views::view(&actions))
    })
    .map_err(|_| {ErrorInternalServerError("Ups")})
    .responder()
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