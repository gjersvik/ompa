use actix_web::{
    App,
    HttpRequest,
    HttpResponse,
    error::ErrorInternalServerError,
    AsyncResponder,
    Error,
};
use futures::future::Future;
use super::{sorter::Sorter,messages::GetActions};

fn index(_req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let res = Sorter::addr().send(GetActions{});

    res.map(|actions| {
        HttpResponse::Ok()
        .content_type("text/html")
        .body(super::views::view(&actions.actions))
    })
    .map_err(|_| {ErrorInternalServerError("Ups")})
    .responder()
}

pub fn app() -> App<()>{
    App::new().resource("/", |r| r.f(index))
}