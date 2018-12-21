mod database;
mod messages;

use self::{
    database::Database,
    messages::{Chore, LoadChores,UpdateTime},
};
use crate::ole_martin::{Action, ActionType, CompletedSub, Priority, UpdateActions,Completed};
use actix::{
    fut::{wrap_future, ActorFuture},
    Actor, Addr, AsyncContext, Context, Recipient, SyncArbiter,Handler,
};
use futures::future::Future;
use std::collections::HashMap;
use chrono::Utc;

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
                    name: name(),
                    actions: chores_to_actions(&actor.chores),
                })
                .unwrap_or_default();
        });

        ctx.spawn(update_self);

        self.completed_sub.do_send(CompletedSub(ctx.address().recipient(), Some(name()))).unwrap_or_default();
    }
}

impl Handler<Completed> for Chores {
    type Result = ();

    fn handle(&mut self, msg: Completed, _: &mut Self::Context) {
        let id = msg.action.index as i32;
        let time = msg.completed;

        // update local
        match self.chores.get_mut(&id){
            Some(chore) => chore.last_done = Some(time),
            None => return
        }

        // send actions
        self.update_action.do_send(UpdateActions {
            name: name(),
            actions: chores_to_actions(&self.chores),
        }).unwrap_or_default();

        // update database
        self.data_store.do_send(UpdateTime(id, time));
    }
}


fn chores_to_actions(chores: &HashMap<i32, Chore>) -> Vec<Action> {
    let now = Utc::now();
    
    chores
        .values()
        .filter_map(|chore| {
            let mut action_type = ActionType::Task(Priority::Important);
            if let Some(last) = chore.last_done {
                if last + chore.frequency > now {
                    return None;
                }
                if last + (chore.frequency * 2) < now {
                    action_type = ActionType::Task(Priority::VeryImportant);
                }
            }

            Some(Action {
                index: chore.id as usize,
                name: chore.name.clone(),
                action_type,
            })
        })
        .collect()
}

fn name() -> String{
    "chores".to_string()
}