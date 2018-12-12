use crate::models::{
    ActionType,
    Priority,
    Action,
    GetActions,
    ActionResult,
};

#[derive(Default)]
pub struct TestActions;

impl actix::Actor for TestActions {
    type Context = actix::Context<Self>;
}

impl actix::Supervised for TestActions {}

impl actix::ArbiterService for TestActions {
   fn service_started(&mut self, _: &mut actix::Context<Self>) {
   }
}

impl actix::Handler<GetActions> for TestActions {
   type Result = ActionResult;

    fn handle(&mut self, _: GetActions, _: &mut actix::Context<Self>) -> Self::Result{
        ActionResult {actions: vec![
            test_action("Play Games", ActionType::Entertainment),
            test_action("Buy new computer", ActionType::Task( Priority::JustForFun)),
            test_action("Print PLA holder", ActionType::Task( Priority::NiceToHave)),
            test_action("Clean old passwords", ActionType::Task( Priority::Useful)),
            test_action("Clean living rom", ActionType::Task( Priority::Important)),
            test_action("Do dishes", ActionType::Task( Priority::VeryImportant)),
            test_action("Clean cloths", ActionType::Task( Priority::Critical)),
            test_action("Go to airport", ActionType::Task( Priority::Mandatory)),
        ]}
    }
}

fn test_action(name: &str, action_type: ActionType) -> Action{
    Action {
        name: name.to_string(),
        description: String::new(),
        action_type,
    }
}