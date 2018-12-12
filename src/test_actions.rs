use actix::{
    Context,
    Actor,
    Supervised,
    ArbiterService,
    Handler,
};

use crate::models::{
    ActionType,
    Priority,
    Action,
    GetActions,
    ActionResult,
};

pub struct TestActions{
    result: ActionResult,
}

impl Default for TestActions {
    fn default() -> TestActions{
        TestActions{ result: Default::default()}
    }
}

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

        self.result.actions = actions.iter().enumerate().map(|(index,(name, action_type))|{
            Action {
                index,
                name: name.to_string(),
                action_type: action_type.clone(),
            }
        }).collect();
    }
}

impl Supervised for TestActions {}

impl ArbiterService for TestActions {
   fn service_started(&mut self, _: &mut Context<Self>) {
   }
}

impl Handler<GetActions> for TestActions {
   type Result = ActionResult;

    fn handle(&mut self, _: GetActions, _: &mut Context<Self>) -> Self::Result{
        self.result.clone()
    }
}