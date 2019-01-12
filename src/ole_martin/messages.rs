use actix::{
    dev::{MessageResponse, ResponseChannel},
    Actor, Message, Recipient,
};
use chrono::{DateTime, Utc};

#[derive(Message)]
pub struct UpdateActions {
    pub name: String,
    pub actions: Vec<Action>,
}

#[derive(Message)]
pub struct CompletedSub(pub Recipient<Completed>, pub Option<String>);

#[derive(Message, Clone)]
pub struct Completed {
    pub started: DateTime<Utc>,
    pub completed: DateTime<Utc>,
    pub source: String,
    pub action: Action,
}

#[derive(Clone)]
pub struct Action {
    pub index: u64,
    pub name: String,
    pub action_type: ActionType,
}

#[derive(Clone)]
pub enum ActionType {
    Entertainment,
    Task(Priority),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Priority {
    /// If you feel like it no problem
    JustForFun,
    /// When you have time and energy to spare
    NiceToHave,
    /// Should be done at some point.
    Useful,
    /// Please to as soon as possible.
    Important,
    /// If you only can do one task today it should be this.
    VeryImportant,
    /// Should be next task.
    Critical,
    /// Must be done NOW!!!
    Mandatory,
}

impl Priority{
    pub fn more_if_possible(&self) -> Priority{
        match self {
            Priority::JustForFun =>  Priority::NiceToHave,
            Priority::NiceToHave =>  Priority::Useful,
            Priority::Useful =>  Priority::Important,
            Priority::Important =>  Priority::VeryImportant,
            Priority::VeryImportant =>  Priority::Critical,
            Priority::Critical =>  Priority::Mandatory,
            Priority::Mandatory =>  Priority::Mandatory,
        }
    }
}

#[derive(Clone, Default)]
pub struct Actions(pub Vec<InternalAction>);

impl<A, M> MessageResponse<A, M> for Actions
where
    A: Actor,
    M: Message<Result = Actions>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

pub struct GetAction(pub String, pub u64);

impl Message for GetAction {
    type Result = Option<InternalAction>;
}

pub struct GetActions;

impl Message for GetActions {
    type Result = Actions;
}

#[derive(Clone)]
pub struct InternalAction {
    pub index: u64,
    pub name: String,
    pub action_type: ActionType,
    pub source: String,
}

impl InternalAction {
    pub fn new(action: Action, source: String) -> InternalAction {
        InternalAction {
            source,
            index: action.index,
            name: action.name,
            action_type: action.action_type,
        }
    }

    pub fn unpack(self) -> (Action, String) {
        (
            Action {
                index: self.index,
                name: self.name,
                action_type: self.action_type,
            },
            self.source,
        )
    }
}

#[derive(Message)]
pub struct StartAction {
    pub action: InternalAction,
    pub time: DateTime<Utc>,
}

#[derive(Message)]
pub struct Done(pub DateTime<Utc>);

#[derive(Message)]
pub struct Cancel;

pub struct GetActive;

impl Message for GetActive {
    type Result = Option<InternalAction>;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn priority_more_if_possible_just_for_fun() {
        assert_eq!(Priority::JustForFun.more_if_possible(), Priority::NiceToHave);
    }
    #[test]
    fn priority_more_if_possible_nice_to_have() {
        assert_eq!(Priority::NiceToHave.more_if_possible(), Priority::Useful);
    }
    #[test]
    fn priority_more_if_possible_useful() {
        assert_eq!(Priority::Useful.more_if_possible(), Priority::Important);
    }
    #[test]
    fn priority_more_if_possible_important() {
        assert_eq!(Priority::Important.more_if_possible(), Priority::VeryImportant);
    }
    #[test]
    fn priority_more_if_possible_very_important() {
        assert_eq!(Priority::VeryImportant.more_if_possible(), Priority::Critical);
    }
    #[test]
    fn priority_more_if_possible_critical() {
        assert_eq!(Priority::Critical.more_if_possible(), Priority::Mandatory);
    }
    #[test]
    fn priority_more_if_possible_mandatory() {
        assert_eq!(Priority::Mandatory.more_if_possible(), Priority::Mandatory);
    }
}
