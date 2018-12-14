mod views;

use actix::{
    Context,
    Actor,
    Addr,
    Supervised,
    SystemService,
    Handler,
    Message,
    System,
    dev::{
        MessageResponse,
        ResponseChannel,
    }
};
use actix_web::{
    App,
    HttpRequest,
    HttpResponse,
    error::ErrorInternalServerError,
    AsyncResponder,
    Error,
};
use futures::future::Future;
use std::{
    collections::HashMap,
    iter::FromIterator,
};

#[derive(Default)]
pub struct OleMartin{
    sources: HashMap<String, Vec<Action>>,
}

impl OleMartin {
    pub fn app() -> App<()>{
        App::new().resource("/", |r| r.f(index))
    }
    pub fn addr() -> Addr<OleMartin> {
        System::current().registry().get::<OleMartin>()
    }
}

impl Actor for OleMartin{
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
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

#[derive(Clone)]
pub struct Action{
    pub index: usize,
    pub name: String,
    pub action_type: ActionType,
}

#[derive(Clone)]
pub enum ActionType{
    Entertainment,
    Task (Priority),
}

#[derive(Clone)]
pub enum Priority{
    /// If you feel like it no problem
    JustForFun,
    /// When you have time and energy to spare
    NiceToHave,
    /// Should be done at some point.
    Useful,
    /// Please to as soon as possible. 
    Important,
    /// If you only can do one task today it should be this.
    VeryImportant,
    /// Should be next task.
    Critical,
    /// Must be done NOW!!!
    Mandatory,
}


fn index(_req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let om = System::current().registry().get::<OleMartin>();

    let res = om.send(GetActions{});

    res.map(|actions| {
        HttpResponse::Ok()
        .content_type("text/html")
        .body(self::views::view(&actions.actions))
    })
    .map_err(|_| {ErrorInternalServerError("Ups")})
    .responder()
}

#[derive(Clone)]
struct ActionResult{
    pub actions: Vec<Action>
}

impl Default for ActionResult {
    fn default() -> ActionResult {
        ActionResult{actions: Vec::new()}
    }
}

impl FromIterator<Action> for ActionResult {
    fn from_iter<I: IntoIterator<Item=Action>>(iter: I) -> Self {
        ActionResult{actions: Vec::from_iter(iter)}
    }
}

impl<A, M> MessageResponse<A, M> for ActionResult
where
    A: Actor,
    M: Message<Result = ActionResult>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}


struct GetActions;

impl Message for GetActions {
    type Result = ActionResult;
}