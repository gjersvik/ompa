use actix_web::{
    App,
    HttpRequest,
    HttpResponse,
    error::ErrorInternalServerError,
    AsyncResponder,
    Error,
    http::header,
};
use futures::future::Future;
use super::{sorter::Sorter,messages::GetActions};

fn index(_req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let res = Sorter::addr().send(GetActions{});

    res.map(|actions| {
        HttpResponse::Ok()
        .content_type("text/html")
        .body(super::views::view(&actions.0))
    })
    .map_err(|_| {ErrorInternalServerError("Ups")})
    .responder()
}

fn start(_req: &HttpRequest) -> HttpResponse{
    HttpResponse::PermanentRedirect().header(header::LOCATION, "/").finish()
}

fn stop(_req: &HttpRequest) -> HttpResponse{
    HttpResponse::PermanentRedirect().header(header::LOCATION, "/").finish()
}

pub fn app() -> App<()>{
    App::new()
        .resource("/", |r| r.f(index))
        .resource("/start", |r| r.f(start))
        .resource("/stop", |r| r.f(stop))

}