use crate::ole_martin::{
    OleMartin,
    UpdateActions,
    Action,
    ActionType,
    Priority,
};
use actix::{
    Context,
    Actor,
    System
};
use futures::future::Future;

#[derive(Default)]
pub struct TestActions;

impl Actor for TestActions {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
       let actions = vec![
            ("Play Games", ActionType::Entertainment),
            ("Buy new computer", ActionType::Task( Priority::JustForFun)),
            ("Print PLA holder", ActionType::Task( Priority::NiceToHave)),
            ("Clean old passwords", ActionType::Task( Priority::Useful)),
            ("Clean living rom", ActionType::Task( Priority::Important)),
            ("Do dishes", ActionType::Task( Priority::VeryImportant)),
            ("Clean cloths", ActionType::Task( Priority::Critical)),
            ("Go to airport", ActionType::Task( Priority::Mandatory)),
        ];

        let actions = actions.iter().enumerate().map(|(index,(name, action_type))|{
            Action {
                index,
                name: name.to_string(),
                action_type: action_type.clone(),
            }
        }).collect();

        let task = System::current().registry().get::<OleMartin>().send(UpdateActions{name: "test".to_string(), actions: actions});
        actix::spawn(task.map(|_|{}).map_err(|_|{}));
    }
}