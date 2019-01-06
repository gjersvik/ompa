use crate::{
    ole_martin::{Action, ActionType, Completed, CompletedSub, Priority, UpdateActions},
    service::todoist::{
        Complete, Item, Items, Priority as TodoistPriority, SubActiveItems,
        Todoist as TodoistService,
    },
};
use actix::{Actor, Addr, AsyncContext, Context, Handler, Recipient};
use chrono::{Duration, Utc};
use im::HashMap;

pub struct Todoist {
    service: Addr<TodoistService>,
    update_action: Recipient<UpdateActions>,
    completed_sub: Recipient<CompletedSub>,
    items: HashMap<u64, Item>,
}

impl Todoist {
    pub fn new(
        service: Addr<TodoistService>,
        update_action: Recipient<UpdateActions>,
        completed_sub: Recipient<CompletedSub>,
    ) -> Todoist {
        Todoist {
            service,
            update_action,
            completed_sub,
            items: HashMap::new(),
        }
    }

    fn send_actions(&self) {
        self.update_action
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
        self.completed_sub
            .do_send(CompletedSub(ctx.address().recipient(), Some(name())))
            .unwrap_or_default();
        self.service
            .do_send(SubActiveItems(ctx.address().recipient()));
    }
}

impl Handler<Items> for Todoist {
    type Result = ();

    fn handle(&mut self, msg: Items, _: &mut Self::Context) {
        self.items = msg.0;
        self.send_actions();
    }
}

impl Handler<Completed> for Todoist {
    type Result = ();

    fn handle(&mut self, msg: Completed, _: &mut Self::Context) {
        if let Some(item) = self.items.remove(&(msg.action.index as u64)) {
            self.send_actions();
            self.service.do_send(Complete(item.id));
        }
    }
}

fn items_to_actions(items: &HashMap<u64, Item>) -> Vec<Action> {
    items
        .values()
        .map(|item| {
            let now = Utc::now();
            let date = item.due_date_utc.map(|d| d - Duration::days(1));

            let priority = match &item.priority {
                TodoistPriority::P4 => match date {
                    Some(date) => {
                        if date <= now {
                            Priority::Useful
                        } else {
                            Priority::NiceToHave
                        }
                    }
                    None => Priority::JustForFun,
                },
                TodoistPriority::P3 => match date {
                    Some(date) => {
                        if date <= now {
                            Priority::Important
                        } else {
                            Priority::Useful
                        }
                    }
                    None => Priority::NiceToHave,
                },
                TodoistPriority::P2 => match date {
                    Some(date) => {
                        if date <= now {
                            Priority::VeryImportant
                        } else {
                            Priority::Important
                        }
                    }
                    None => Priority::Useful,
                },
                TodoistPriority::P1 => match date {
                    Some(date) => {
                        if date <= now {
                            Priority::Critical
                        } else {
                            Priority::VeryImportant
                        }
                    }
                    None => Priority::Important,
                },
            };

            Action {
                index: item.id as usize,
                name: item.content.clone(),
                action_type: ActionType::Task(priority),
            }
        })
        .collect()
}

fn name() -> String {
    "todoist".to_string()
}
