use actix::{
    Actor,
    Message,
    dev::{
        MessageResponse,
        ResponseChannel,
    }
};
use chrono::{DateTime, Utc};

#[derive(Message)]
pub struct UpdateActions{
    pub name: String,
    pub actions: Vec<Action>
}

#[derive(Clone)]
pub struct Action{
    pub index: usize,
    pub name: String,
    pub action_type: ActionType,
}

#[derive(Clone)]
pub enum ActionType{
    Entertainment,
    Task (Priority),
}

#[derive(Clone)]
pub enum Priority{
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

pub struct GetAction(pub String, pub usize);

impl Message for GetAction {
    type Result = Option<InternalAction>;
}


pub struct GetActions;

impl Message for GetActions {
    type Result = Actions;
}

#[derive(Clone)]
pub struct InternalAction {
    pub index: usize,
    pub name: String,
    pub action_type: ActionType,
    pub source: String,
}

impl InternalAction{
    pub fn new(action: Action, source: String) -> InternalAction{
        InternalAction{
            source,
            index: action.index,
            name: action.name,
            action_type: action.action_type,
        }
    }
}

#[derive(Message)]
pub struct StartAction{
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