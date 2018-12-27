mod messages;
mod service;

use self::{messages::{Sync, Complete}, service::Service};
use crate::ole_martin::{Action, ActionType, Completed, CompletedSub, Priority, UpdateActions};
use actix::{
    fut::{wrap_future, ActorFuture},
    Actor, Addr, AsyncContext, Context, Handler, Recipient, SyncArbiter,
};
use chrono::Utc;
use futures::future::Future;
use std::{collections::HashMap, time::Duration};
use todoist::{resource::Item, IntBool};

pub struct Todoist {
    items: HashMap<usize, Item>,
    sync_token: Option<String>,
    service: Addr<Service>,
    update_action: Recipient<UpdateActions>,
    completed_sub: Recipient<CompletedSub>,
}

impl Todoist {
    pub fn new(
        token: String,
        update_action: Recipient<UpdateActions>,
        completed_sub: Recipient<CompletedSub>,
    ) -> Todoist {
        Todoist {
            items: HashMap::new(),
            sync_token: None,
            service: SyncArbiter::start(1, move || Service::new(token.clone())),
            update_action,
            completed_sub,
        }
    }

    fn update(&mut self, ctx: &mut Context<Self>) {
        let load = self.service.send(Sync(None)).map_err(|_| {});

        let update_self = wrap_future::<_, Self>(load).map(|res, actor, _| {
            actor.sync_token = res.sync_token;
            if res.items.is_empty() {
                return;
            }
            for item in res.items {
                actor.items.insert(item.id, item);
            }
            actor.send_actions();
        });

        ctx.spawn(update_self);
    }

    fn send_actions(&self){
        self
            .update_action
            .do_send(UpdateActions {
                name: name(),
                actions: items_to_actions(&self.items),
            })
            .unwrap_or_default();
    }
}

impl Actor for Todoist {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        self.update(ctx);

        self.completed_sub.do_send(CompletedSub(ctx.address().recipient(), Some(name()))).unwrap_or_default();

        ctx.run_interval(Duration::from_secs(5 * 60), Self::update);
    }
}

impl Handler<Completed> for Todoist {
    type Result = ();

    fn handle(&mut self, msg: Completed, ctx: &mut Self::Context) {
        let id = msg.action.index;
        // update local 
        if let Some(item) = self.items.get_mut(&id){
            item.checked = IntBool::from(true);
        } else {
            return;
        }
        self.send_actions();

        // update and sync with service.
        let update = self.service.send(Complete(id)).map_err(|_| {});
        let update_self = wrap_future::<_, Self>(update).map(|_, actor, ctx| {
            actor.update(ctx);
        });
        ctx.spawn(update_self);
    }
}

fn items_to_actions(items: &HashMap<usize, Item>) -> Vec<Action> {
    items
        .values()
        .filter_map(|item| {
            if item.checked == 1 {
                return None;
            }

            let today = Utc::today().and_hms(0, 0, 0);
            let date = item
                .due_date_utc
                .clone()
                .map(|d| d.timestamp.with_timezone(&Utc));

            let priority = match item.priority {
                1 => match date {
                    Some(date) => {
                        if date < today {
                            Priority::Useful
                        } else {
                            Priority::NiceToHave
                        }
                    }
                    None => Priority::JustForFun,
                },
                2 => match date {
                    Some(date) => {
                        if date <= today {
                            Priority::Important
                        } else {
                            Priority::Useful
                        }
                    }
                    None => Priority::NiceToHave,
                },
                3 => match date {
                    Some(date) => {
                        if date <= today {
                            Priority::VeryImportant
                        } else {
                            Priority::Important
                        }
                    }
                    None => Priority::Useful,
                },
                _ => match date {
                    Some(date) => {
                        if date <= today {
                            Priority::Critical
                        } else {
                            Priority::VeryImportant
                        }
                    }
                    None => Priority::Important,
                },
            };

            Some(Action {
                index: item.id,
                name: item.content.clone().unwrap_or_default(),
                action_type: ActionType::Task(priority),
            })
        })
        .collect()
}

fn name() -> String {
    "todoist".to_string()
}
