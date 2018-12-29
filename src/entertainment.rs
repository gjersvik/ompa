use crate::ole_martin::{Action, ActionType, UpdateActions};
use actix::Recipient;

pub fn send(addr: &Recipient<UpdateActions>){
    let mut actions = Vec::new();

    actions.push(create_action(0,"Watch Netflix"));
    actions.push(create_action(1,"Play PS4"));
    actions.push(create_action(2,"Watch Movie"));
    actions.push(create_action(3,"YouTube and TM"));
    actions.push(create_action(4,"YouTube and Idle"));
    actions.push(create_action(5,"Play pc games"));

    addr.do_send(UpdateActions{name: "entertainment".to_string() ,actions}).unwrap_or_default();
}

fn create_action(index: usize, name: &str,)-> Action{
    Action {
        index,
        name: name.to_string(),
        action_type: ActionType::Entertainment,
    }
}