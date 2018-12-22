use crate::ole_martin::{Action, ActionType, UpdateActions};
use actix::Recipient;

pub fn send(addr: &Recipient<UpdateActions>){
    let mut actions = Vec::new();

    actions.push(create_action("Watch Netflix"));
    actions.push(create_action("Play PS4"));
    actions.push(create_action("Watch Movie"));
    actions.push(create_action("YouTube and TM"));
    actions.push(create_action("YouTube and Idle"));
    actions.push(create_action("Play pc games"));

    addr.do_send(UpdateActions{name: "entertainment".to_string() ,actions}).unwrap_or_default();
}

fn create_action(name: &str)-> Action{
    Action {
        index: 0,
        name: name.to_string(),
        action_type: ActionType::Entertainment,
    }
}