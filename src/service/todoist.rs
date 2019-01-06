mod messages;
mod service;

use self::{
    messages::{Complete as SComplete, Sync},
    service::Service,
};
use actix::{
    fut::{wrap_future, ActorFuture},
    Actor, Addr, AsyncContext, Context, Handler, Message, Recipient, SyncArbiter,
};
use chrono::{DateTime, Utc};
use futures::future::Future;
use im::HashMap;
use std::time::Duration;
use todoist::resource::Item as SItem;

#[derive(Message)]
pub struct SubActiveItems(pub Recipient<Items>);

#[derive(Message)]
pub struct Items(pub HashMap<u64, Item>);

#[derive(Clone)]
pub struct Item {
    pub id: u64,
    pub content: String,
    pub due_date_utc: Option<DateTime<Utc>>,
    pub priority: Priority,
}

impl Item {
    fn from_back_end(item: SItem) -> Item {
        Item {
            id: item.id as u64,
            content: item.content.unwrap_or_default(),
            due_date_utc: item.due_date_utc.map(|d| d.timestamp.with_timezone(&Utc)),
            priority: Priority::from_back_end(item.priority),
        }
    }
}

#[derive(Clone)]
pub enum Priority {
    P1, // Most important
    P2,
    P3,
    P4, // Least important
}

impl Priority {
    fn from_back_end(p: u8) -> Priority {
        match p {
            4 => Priority::P1,
            3 => Priority::P2,
            2 => Priority::P3,
            _ => Priority::P4,
        }
    }
}

#[derive(Message)]
pub struct Complete(pub u64);

pub struct Todoist {
    items: HashMap<u64, Item>,
    service: Addr<Service>,
    subs: Vec<Recipient<Items>>,
}

impl Todoist {
    pub fn new(token: String) -> Todoist {
        Todoist {
            items: HashMap::new(),
            service: SyncArbiter::start(1, move || Service::new(token.clone())),
            subs: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &mut Context<Self>) {
        let load = self.service.send(Sync(None)).map_err(|_| {});

        let update_self = wrap_future::<_, Self>(load).map(|res, actor, _| {
            if res.items.is_empty() {
                return;
            }
            actor.set_times(res.items);
            actor.send_actions();
        });

        ctx.spawn(update_self);
    }

    fn set_times(&mut self, items: Vec<SItem>) {
        self.items = HashMap::new();

        for item in items {
            if item.checked == 1 {
                continue;
            }
            if item.is_deleted == 1 {
                continue;
            }
            if item.is_archived == 1 {
                continue;
            }

            let new_item = Item::from_back_end(item);

            self.items.insert(new_item.id, new_item);
        }
    }

    fn send_actions(&self) {
        for sub in &self.subs {
            sub.do_send(Items(self.items.clone())).unwrap_or_default();
        }
    }
}

impl Actor for Todoist {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        self.update(ctx);
        ctx.run_interval(Duration::from_secs(5 * 60), Self::update);
    }
}

impl Handler<SubActiveItems> for Todoist {
    type Result = ();

    fn handle(&mut self, msg: SubActiveItems, _: &mut Self::Context) {
        msg.0.do_send(Items(self.items.clone())).unwrap_or_default();
        self.subs.push(msg.0);
    }
}

impl Handler<Complete> for Todoist {
    type Result = ();

    fn handle(&mut self, msg: Complete, ctx: &mut Self::Context) {
        // update and sync with service.
        let update = self.service.send(SComplete(msg.0 as usize)).map_err(|_| {});
        let update_self = wrap_future::<_, Self>(update).map(|_, actor, ctx| {
            actor.update(ctx);
        });
        ctx.spawn(update_self);
    }
}
