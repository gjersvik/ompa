use crate::{
    ole_martin::{Action, ActionType, Completed, CompletedSub, Priority, UpdateActions},
    service::todoist::{
        Complete, Item, Items, Priority as TodoistPriority, SubActiveItems,
        Todoist as TodoistService,
    },
};
use actix::{Actor, Addr, AsyncContext, Context, Handler, Recipient};
use chrono::{Duration, Utc, DateTime};
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
                name: NAME.to_string(),
                actions: items_to_actions(&self.items, &Utc::now()),
            })
            .unwrap_or_default();
    }
}

impl Actor for Todoist {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        self.completed_sub
            .do_send(CompletedSub(ctx.address().recipient(), Some(NAME.to_string())))
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

const NAME: &str = "todoist";

impl Item {
    fn to_action(&self, now: &DateTime<Utc>) -> Action{
        Action {
            index: self.id,
            name: self.content.clone(),
            action_type: ActionType::Task(self.get_action_priority(now)),
        }
    }

    fn get_action_priority(&self, now: &DateTime<Utc>) -> Priority{
        let mut priority = match &self.priority {
            TodoistPriority::P4 => Priority::JustForFun,
            TodoistPriority::P3 => Priority::NiceToHave,
            TodoistPriority::P2 => Priority::Useful,
            TodoistPriority::P1 => Priority::Important,
        };

        if let Some(date) = self.due_date_utc {
            priority = priority.more_if_possible();
            if date <= *now + Duration::days(1) {
                priority = priority.more_if_possible();
            }
        }

        priority
    }
}

fn items_to_actions(items: &HashMap<u64, Item>, now: &DateTime<Utc>) -> Vec<Action> {
    items
        .values()
        .map(|item| item.to_action(now))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::offset::TimeZone;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref NOW:DateTime<Utc> = Utc.ymd(2010,1,1).and_hms(0,0,0);
    }

    fn test_priority(pri: TodoistPriority) -> Priority{
        test_priority_base(pri, None)
    }

    fn test_priority_date(pri: TodoistPriority) -> Priority{
        test_priority_base(pri, Some(*NOW + Duration::days(1) + Duration::seconds(1)))
    }

    fn test_priority_last_day(pri: TodoistPriority) -> Priority{
        test_priority_base(pri, Some(*NOW + Duration::days(1)))
    }

    fn test_priority_base(pri: TodoistPriority, due: Option<DateTime<Utc>>) -> Priority {
        let item = Item{
            id:0,
            content: "test".to_string(),
            due_date_utc: due,
            priority: pri,
        };

        if let ActionType::Task(pri) = item.to_action(&NOW).action_type {
            Some(pri)
        } else {
            None
        }.unwrap()
    }

    #[test]
    fn item_get_action_priority_p1() {
        assert_eq!(test_priority(TodoistPriority::P1), Priority::Important);
    }

    #[test]
    fn item_get_action_priority_p2() {
        assert_eq!(test_priority(TodoistPriority::P2), Priority::Useful);
    }

    #[test]
    fn item_get_action_priority_p3() {
        assert_eq!(test_priority(TodoistPriority::P3), Priority::NiceToHave);
    }

    #[test]
    fn item_get_action_priority_p4() {
        assert_eq!(test_priority(TodoistPriority::P4), Priority::JustForFun);
    }

    #[test]
    fn item_get_action_priority_p1_date() {
        assert_eq!(test_priority_date(TodoistPriority::P1), Priority::VeryImportant);
    }

    #[test]
    fn item_get_action_priority_p2_date() {
        assert_eq!(test_priority_date(TodoistPriority::P2), Priority::Important);
    }

    #[test]
    fn item_get_action_priority_p3_date() {
        assert_eq!(test_priority_date(TodoistPriority::P3), Priority::Useful);
    }

    #[test]
    fn item_get_action_priority_p4_date() {
        assert_eq!(test_priority_date(TodoistPriority::P4), Priority::NiceToHave);
    }

    #[test]
    fn item_get_action_priority_p1_last_day() {
        assert_eq!(test_priority_last_day(TodoistPriority::P1), Priority::Critical);
    }

    #[test]
    fn item_get_action_priority_p2_last_day() {
        assert_eq!(test_priority_last_day(TodoistPriority::P2), Priority::VeryImportant);
    }

    #[test]
    fn item_get_action_priority_p3_last_day() {
        assert_eq!(test_priority_last_day(TodoistPriority::P3), Priority::Important);
    }

    #[test]
    fn item_get_action_priority_p4_last_day() {
        assert_eq!(test_priority_last_day(TodoistPriority::P4), Priority::Useful);
    }

}