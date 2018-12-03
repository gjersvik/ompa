#[derive(Serialize)]
pub struct Action{
    pub name: String,
    pub description: String,
    pub action_type: ActionType,
}

#[derive(Serialize)]
pub enum ActionType{
    Entertainment,
    Task(Priority),
}

#[allow(dead_code)]
#[derive(Serialize)]
pub enum Priority{
    NiceToHave,
    AtSomePoint,
    Important,
    VeryImportant,
    Critical,
    Mandatory,
}