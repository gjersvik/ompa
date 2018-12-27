mod messages;
mod service;

use self::{
    service::Service,
    messages::Sync,
};
use crate::ole_martin::{Action, ActionType, CompletedSub, Priority, UpdateActions,Completed};
use actix::{
    fut::{wrap_future, ActorFuture},
    Actor, Addr, AsyncContext, Context, Recipient, SyncArbiter,Handler,
};
use futures::future::Future;
use todoist::resource::Item;
use std::{
    collections::HashMap,
    time::Duration,
};
use chrono::Utc;

pub struct Todoist{
    items: HashMap<usize, Item>,
    sync_token: Option<String>,
    service: Addr<Service>,
    update_action: Recipient<UpdateActions>,
}

impl Todoist {
    pub fn new(token: String, update_action: Recipient<UpdateActions>) -> Todoist {
        Todoist {
            items: HashMap::new(),
            sync_token: None,
            service: SyncArbiter::start(1, move|| Service::new(token.clone())),
            update_action,
        }
    }

    fn update(&mut self, ctx: &mut Context<Self>){
        let load = self.service.send(Sync(None)).map_err(|_| {});

        let update_self = wrap_future::<_, Self>(load).map(|res, actor, _| {
            actor.sync_token = res.sync_token;
            if res.items.is_empty() {
                return;
            }
            for item in res.items {
                actor.items.insert(item.id, item);
            }
            actor.update_action.do_send(UpdateActions {
                name: name(),
                actions: items_to_actions(&actor.items),
            }).unwrap_or_default();
        });

        ctx.spawn(update_self);
    }
}

impl Actor for Todoist {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        self.update(ctx);

        ctx.run_interval(Duration::from_secs(5*60), Self::update);
    }
}

fn items_to_actions(items: &HashMap<usize, Item>) -> Vec<Action> {
    items.values().filter_map(|item| {
        if item.checked == 1 {
            return None
        }

        let today = Utc::today().and_hms(0,0,0);
        let date = item.due_date_utc.clone().map(|d| d.timestamp.with_timezone(&Utc));

        let priority = match item.priority {
            1 =>  {
                match date {
                    Some(date) => {
                        if date < today {
                            Priority::Useful
                        }else {
                            Priority::NiceToHave
                        }
                    }
                    None => Priority::JustForFun,
                }
            },
            2 =>{
                match date {
                    Some(date) => {
                        if date <= today {
                            Priority::Important
                        }else {
                            Priority::Useful
                        }
                    }
                    None => Priority::NiceToHave,
                }
            },
            3 =>{
                match date {
                    Some(date) => {
                        if date <= today {
                            Priority::VeryImportant
                        }else {
                            Priority::Important
                        }
                    }
                    None => Priority::Useful,
                }
            },
            _ =>{
                match date {
                    Some(date) => {
                        if date <= today {
                            Priority::Critical
                        }else {
                            Priority::VeryImportant
                        }
                    }
                    None => Priority::Important,
                }
            },
        };

        Some(Action {
            index: item.id,
            name: item.content.clone().unwrap_or_default(),
            action_type: ActionType::Task(priority),
        })
    }).collect()
}

fn name() -> String{
    "todoist".to_string()
}