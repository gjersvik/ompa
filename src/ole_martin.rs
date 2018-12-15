mod handlers;
mod messages;
mod notifier;
mod sorter;
mod tracker;
mod views;

use actix::{
    Context,
    Actor,
    Addr,
    Handler,
};
use actix_web::App;

use self::{
    sorter::Sorter,
    notifier::Notifier,
};

pub use self::messages::{Action, UpdateActions, ActionType, Priority, CompletedSub, Completed};

#[derive(Default)]
pub struct OleMartin;

impl OleMartin {
    pub fn app() -> App<()>{
        handlers::app()
    }
    pub fn addr() -> Addr<OleMartin> {
        OleMartin.start()
    }
}

impl Actor for OleMartin{
    type Context = Context<Self>;
}

impl Handler<UpdateActions> for OleMartin {
    type Result = ();

    fn handle(&mut self, msg: UpdateActions, _: &mut Self::Context) {
        Sorter::addr().do_send(msg);
    }
}

impl Handler<CompletedSub> for OleMartin {
    type Result = ();

    fn handle(&mut self, msg: CompletedSub, _: &mut Self::Context) {
        Notifier::addr().do_send(msg);
    }
}