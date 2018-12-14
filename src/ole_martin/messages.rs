use actix::{
    Actor,
    Message,
    dev::{
        MessageResponse,
        ResponseChannel,
    }
};
use std::{
    iter::FromIterator,
};

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

#[derive(Clone)]
pub struct ActionResult{
    pub actions: Vec<Action>
}

impl Default for ActionResult {
    fn default() -> ActionResult {
        ActionResult{actions: Vec::new()}
    }
}

impl FromIterator<Action> for ActionResult {
    fn from_iter<I: IntoIterator<Item=Action>>(iter: I) -> Self {
        ActionResult{actions: Vec::from_iter(iter)}
    }
}

impl<A, M> MessageResponse<A, M> for ActionResult
where
    A: Actor,
    M: Message<Result = ActionResult>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

pub struct GetActions;

impl Message for GetActions {
    type Result = ActionResult;
}