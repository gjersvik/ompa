use actix::{
    Context,
    Actor,
    Addr,
    Supervised,
    SystemService,
    Handler,
    System,
    AsyncContext,
};

use super::ole_martin::{Completed, CompletedSub, OleMartin};

#[derive(Default)]
pub struct Logger;

impl Logger {
    pub fn addr() -> Addr<Logger> {
         System::current().registry().get::<Logger>()
    }
}

impl Actor for Logger{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
         OleMartin::addr().do_send(CompletedSub(ctx.address().recipient(),None));
    }
}

impl Supervised for Logger {}

impl SystemService for Logger {}

impl Handler<Completed> for Logger {
    type Result = ();

    fn handle(&mut self, msg: Completed, _: &mut Self::Context) {
        println!("{} completed from {}", msg.action.name, msg.source);
        println!("Duration from {:?} to {:?}", msg.started, msg.completed);
    }
}