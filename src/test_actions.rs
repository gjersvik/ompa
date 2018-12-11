use crate::models::{
    ActionType,
    Priority,
    Action
};

pub struct GetActions;

impl actix::Message for GetActions {
    type Result = Result<Vec<Action>,()>;
}

#[derive(Default)]
pub struct TestActions;

impl actix::Actor for TestActions {
    type Context = actix::Context<Self>;
}

impl actix::Supervised for TestActions {}

impl actix::ArbiterService for TestActions {
   fn service_started(&mut self, ctx: &mut actix::Context<Self>) {
   }
}

impl actix::Handler<GetActions> for TestActions {
   type Result = Result<Vec<Action>,()>;

    fn handle(&mut self, _: GetActions, _: &mut actix::Context<Self>) -> Self::Result{
        Ok(vec![
            test_action("Play Games", ActionType::Entertainment),
            test_action("Do dishes", ActionType::Task( Priority::Important)),
        ])
    }
}

fn test_action(name: &str, action_type: ActionType) -> Action{
    Action {
        name: name.to_string(),
        description: String::new(),
        action_type,
    }
}