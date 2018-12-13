use actix::{
    Context,
    Actor,
    Supervised,
    SystemService,
    Handler,
    Message,
    System,
};
use actix_web::{
    server,
    App,
    HttpRequest,
    HttpResponse,
    error::ErrorInternalServerError,
    AsyncResponder,
    Error,
};
use futures::future::Future;
use std::collections::HashMap;

use crate::models::{GetActions, ActionResult, Action};

#[derive(Default)]
pub struct OleMartin{
    sources: HashMap<String, Vec<Action>>,
}

impl Actor for OleMartin{
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
        let server = server::new(|| App::new().resource("/", |r| r.f(index)));
        let addr = "127.0.0.1:7878";
        println!("Listening for requests at http://{}", addr);
        let server = server.bind(addr).unwrap();
        server.start();
    }
}

impl Supervised for OleMartin {}

impl SystemService for OleMartin {
   fn service_started(&mut self, _: &mut Context<Self>) {
   }
}

impl Handler<UpdateActions> for OleMartin {
    type Result = ();

    fn handle(&mut self, msg: UpdateActions, _: &mut Self::Context) {
        self.sources.insert(msg.name, msg.actions);
    }
}

impl Handler<GetActions> for OleMartin {
    type Result = ActionResult;

    fn handle(&mut self, _: GetActions, _: &mut Self::Context) -> Self::Result {
        let actions = self.sources.iter().flat_map(|(_, source)| source).cloned().collect();
        ActionResult{actions}
    }
}

#[derive(Message)]
pub struct UpdateActions{
    pub name: String,
    pub actions: Vec<Action>
}

fn index(_req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let om = System::current().registry().get::<OleMartin>();

    let res = om.send(GetActions{});

    res.map(|actions| {
        HttpResponse::Ok()
        .content_type("text/html")
        .body(crate::views::view(&actions.actions))
    })
    .map_err(|_| {ErrorInternalServerError("Ups")})
    .responder()
}