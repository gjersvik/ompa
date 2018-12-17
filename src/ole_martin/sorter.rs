use actix::{Actor, Addr, Context, Handler, Supervised, System, SystemService};
use std::collections::HashMap;

use super::{
    messages::{Actions, GetAction, GetActions, InternalAction},
    Action, UpdateActions,
};

#[derive(Default)]
pub struct Sorter {
    sources: HashMap<String, Vec<Action>>,
}

impl Sorter {
    pub fn addr() -> Addr<Sorter> {
        System::current().registry().get::<Sorter>()
    }
}

impl Actor for Sorter {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {}
}

impl Supervised for Sorter {}

impl SystemService for Sorter {
    fn service_started(&mut self, _: &mut Context<Self>) {}
}

impl Handler<UpdateActions> for Sorter {
    type Result = ();

    fn handle(&mut self, msg: UpdateActions, _: &mut Self::Context) {
        self.sources.insert(msg.name, msg.actions);
    }
}

impl Handler<GetActions> for Sorter {
    type Result = Actions;

    fn handle(&mut self, _: GetActions, _: &mut Self::Context) -> Self::Result {
        let actions = self.sources.iter().flat_map(to_internal).collect();
        Actions(actions)
    }
}

impl Handler<GetAction> for Sorter {
    type Result = Option<InternalAction>;

    fn handle(&mut self, msg: GetAction, _: &mut Self::Context) -> Self::Result {
        let actions = self.sources.get(&msg.0)?;
        let action = actions.iter().find(|a| a.index == msg.1)?;

        Some(InternalAction::new(action.clone(), msg.0))
    }
}

fn to_internal(kv: (&String, &Vec<Action>)) -> Vec<InternalAction> {
    let (name, actions) = kv;
    actions
        .iter()
        .map(|a| InternalAction::new(a.clone(), name.clone()))
        .collect()
}
