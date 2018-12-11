use crate::models::{Action, ActionType, Priority};
use tera::{Context, Tera, compile_templates};
use lazy_static::lazy_static;

/// Load in templates from the project template folder.
lazy_static! {
    static ref TERA: Tera = {
        let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
        tera
    };
}

pub fn view(list: &[Action]) -> String{
    let context = get_context(list);
    TERA.render("index.html.tera", &context).unwrap()
}

fn get_context(list: &[Action]) -> Context{
    let actions:Vec<Context> = list.iter().map(action_context).collect();
    let mut context = Context::new();
    context.insert("actions", &actions);
    context
}

fn action_context(action:&Action) -> Context{
    let mut context = Context::new();
    context.insert("name", &action.name);
    context.insert("type", print_action_type(&action.action_type));
    context
}

fn print_action_type(action: &ActionType) -> &'static str{
    match action {
        ActionType::Entertainment => "entertainment",
        ActionType::Task(Priority::JustForFun) => "just for fun task",
        ActionType::Task(Priority::NiceToHave) => "nice to have task",
        ActionType::Task(Priority::Useful) => "useful task",
        ActionType::Task(Priority::Important) => "important task",
        ActionType::Task(Priority::VeryImportant) => "very important task",
        ActionType::Task(Priority::Critical) => "critical task",
        ActionType::Task(Priority::Mandatory) => "mandatory task",
    }
}