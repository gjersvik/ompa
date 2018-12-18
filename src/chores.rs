mod database;
mod messages;

use self::{
    database::Database,
    messages::{Chore, LoadChores},
};
use crate::ole_martin::{Action, ActionType, CompletedSub, Priority, UpdateActions};
use actix::{
    fut::{wrap_future, ActorFuture},
    Actor, Addr, AsyncContext, Context, Recipient, SyncArbiter,
};
use futures::future::Future;
use std::collections::HashMap;

pub struct Chores {
    chores: HashMap<i32, Chore>,
    update_action: Recipient<UpdateActions>,
    completed_sub: Recipient<CompletedSub>,
    data_store: Addr<Database>,
}

impl Chores {
    pub fn new(
        db_uri: String,
        update_action: Recipient<UpdateActions>,
        completed_sub: Recipient<CompletedSub>,
    ) -> Chores {
        Chores {
            chores: HashMap::new(),
            update_action,
            completed_sub,
            data_store: SyncArbiter::start(1, move || Database::new(db_uri.clone())),
        }
    }
}

impl Actor for Chores {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        let load = self.data_store.send(LoadChores).map_err(|_| {});

        let update_self = wrap_future::<_, Self>(load).map(|chores, actor, _| {
            actor.chores = chores.0;
            actor
                .update_action
                .do_send(UpdateActions {
                    name: "chores".to_string(),
                    actions: chores_to_actions(&actor.chores),
                })
                .unwrap_or_default();
        });

        ctx.spawn(update_self);
    }
}

fn chores_to_actions(chores: &HashMap<i32, Chore>) -> Vec<Action> {
    chores
        .values()
        .map(|chore| Action {
            index: chore.id as usize,
            name: chore.name.clone(),
            action_type: ActionType::Task(Priority::Important),
        })
        .collect()
}
