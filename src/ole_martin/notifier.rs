use actix::{Actor, Addr, Context, Handler, Recipient, Supervised, System, SystemService};
use std::collections::HashMap;

use super::messages::{Completed, CompletedSub};

#[derive(Default)]
pub struct Notifier {
    topics: HashMap<String, Vec<Recipient<Completed>>>,
    all: Vec<Recipient<Completed>>,
}

impl Notifier {
    pub fn addr() -> Addr<Notifier> {
        System::current().registry().get::<Notifier>()
    }
}

impl Actor for Notifier {
    type Context = Context<Self>;
}

impl Supervised for Notifier {}

impl SystemService for Notifier {}

impl Handler<CompletedSub> for Notifier {
    type Result = ();

    fn handle(&mut self, msg: CompletedSub, _: &mut Self::Context) {
        match msg.1 {
            Some(key) => match self.topics.get_mut(&key) {
                Some(list) => list.push(msg.0),
                None => {
                    self.topics.insert(key, vec![msg.0]);
                }
            },
            None => self.all.push(msg.0),
        }
    }
}

impl Handler<Completed> for Notifier {
    type Result = ();

    fn handle(&mut self, msg: Completed, _: &mut Self::Context) {
        let empty = Vec::new();
        let topic = self.topics.get(&msg.source).unwrap_or(&empty);
        for addr in topic {
            addr.do_send(msg.clone()).unwrap_or_default();
        }
        for addr in self.all.iter() {
            addr.do_send(msg.clone()).unwrap_or_default();
        }
    }
}
