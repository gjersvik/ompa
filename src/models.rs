use actix::{
    Actor,
    Message,
    dev::{
        MessageResponse,
        ResponseChannel,
    }
};

pub struct Action{
    pub name: String,
    pub description: String,
    pub action_type: ActionType,
}

pub enum ActionType{
    Entertainment,
    Task (Priority),
}

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

pub struct ActionResult{
    pub actions: Vec<Action>
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