use models::ActionType;
use models::Priority;
use models::Action;

pub fn test_actions() -> Vec<Action> {
    vec![
        test_action("Play Games", ActionType::Entertainment),
        test_action("Do dishes", ActionType::Task( Priority::Important)),
    ]
}

fn test_action(name: &str, action_type: ActionType) -> Action{
    Action {
        name: name.to_string(),
        description: String::new(),
        action_type: action_type,
    }
}