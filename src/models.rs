use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Action{
    pub name: String,
    pub description: String,
    pub action_type: ActionType,
}

#[derive(Serialize)]
pub enum ActionType{
    Entertainment,
    Task (Priority),
}

#[allow(dead_code)]
#[derive(Serialize)]
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