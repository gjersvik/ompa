use actix::{
    Context,
    Actor,
    Addr,
    Supervised,
    SystemService,
    Handler,
    System,
};
use chrono::{DateTime, Utc};

use super::{
    messages::{InternalAction, StartAction, Done, Cancel, GetActive},
};

pub struct Tracker{
    active: Option<InternalAction>,
    start_time: DateTime<Utc>
}

impl Default for Tracker {
    fn default() -> Tracker {
        Tracker{ active: None, start_time: Utc::now()}
    }
}

impl Tracker {
    pub fn addr() -> Addr<Tracker> {
        System::current().registry().get::<Tracker>()
    }
}

impl Actor for Tracker{
    type Context = Context<Self>;
}

impl Supervised for Tracker {}

impl SystemService for Tracker {}

impl Handler<StartAction> for Tracker {
    type Result = ();

    fn handle(&mut self, msg: StartAction, _: &mut Self::Context) {
        self.active = Some(msg.action);
        self.start_time = msg.time;
    }
}

impl Handler<Done> for Tracker {
    type Result = ();

    fn handle(&mut self, _: Done, _: &mut Self::Context) {
        if let Some(active) = &self.active {
            println!("{} is done", active.name);
            self.active = None;
        }
    }
}

impl Handler<Cancel> for Tracker {
    type Result = ();

    fn handle(&mut self, _: Cancel, _: &mut Self::Context) {
        self.active = None;
    }
}

impl Handler<GetActive> for Tracker {
    type Result = Option<InternalAction>;

    fn handle(&mut self, _: GetActive, _: &mut Self::Context) -> Self::Result {
        self.active.clone()
    }
}