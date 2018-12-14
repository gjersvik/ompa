use actix_web::{
    App,
    HttpRequest,
    HttpResponse,
    error::ErrorInternalServerError,
    AsyncResponder,
    Error,
    Form,
    http::header,
    FromRequest,
    FutureResponse
};
use futures::{
    future,
    future::Future,
};
use serde_derive::Deserialize;
use chrono::{Utc,DateTime};

use super::{
    sorter::Sorter,
    tracker::Tracker,
    messages::{GetActions, GetAction, StartAction, Done, Cancel,GetActive},
};

fn index(_req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let res = Sorter::addr().send(GetActions{}).join(Tracker::addr().send(GetActive));

    res.map(|(actions, active)| {
        HttpResponse::Ok()
        .content_type("text/html")
        .body(super::views::view(&actions.0, active))
    })
    .map_err(|_| {ErrorInternalServerError("Ups")})
    .responder()
}

#[derive(Deserialize)]
struct StartForm {
    id: usize,
    source: String,
}

fn start(req: &HttpRequest) -> FutureResponse<HttpResponse, Error>{
    let time = Utc::now();
    Form::<StartForm>::extract(req).map_err(|_| ())
        .and_then(|f|{
            Sorter::addr().send(GetAction(f.source.clone(), f.id)).map_err(|_| ())
        }).and_then(|o| o.ok_or(())).and_then(move |a|{
            Tracker::addr().send(StartAction{action: a, time}).map_err(|_| ())
        }).then(|_| {
            future::ok(HttpResponse::PermanentRedirect().header(header::LOCATION, "/").finish())
        }).responder()
}

#[derive(Deserialize)]
struct StopForm {
    done: Option<String>,
    cancel: Option<String>,
}
fn stop(req: &HttpRequest) -> FutureResponse<HttpResponse, Error>{
    let time = Utc::now();
    Form::<StopForm>::extract(req)
        .and_then(move |f|{
            if f.done.is_some() {
                done(time)
            }else if f.cancel.is_some() {
                cancel()
            }else{
                future::ok(HttpResponse::PermanentRedirect().header(header::LOCATION, "/").finish()).responder()
            }
        }).responder()
}

fn done(time: DateTime<Utc>) -> FutureResponse<HttpResponse, Error>{
    Tracker::addr().send(Done(time)).then(|_| {
        future::ok(HttpResponse::PermanentRedirect().header(header::LOCATION, "/").finish())
    }).responder()
}

fn cancel() -> FutureResponse<HttpResponse, Error>{
    Tracker::addr().send(Cancel).then(|_| {
        future::ok(HttpResponse::PermanentRedirect().header(header::LOCATION, "/").finish())
    }).responder()
}
pub fn app() -> App<()>{
    App::new()
        .resource("/", |r| r.f(index))
        .resource("/start", |r| r.f(start))
        .resource("/stop", |r| r.f(stop))

}