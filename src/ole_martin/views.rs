use super::messages::{InternalAction, ActionType, Priority};
use tera::{Context, Tera, compile_templates};
use lazy_static::lazy_static;

/// Load in templates from the project template folder.
lazy_static! {
    static ref TERA: Tera = {
        let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
        tera
    };
}

pub fn view(list: &[InternalAction], op: Option<InternalAction>) -> String{
    let mut context = get_context(list);
    if let Some(op) = op {
        context.insert("active", &action_context(&op));
    }
    TERA.render("index.html.tera", &context).unwrap()
}

fn get_context(list: &[InternalAction]) -> Context{
    let actions:Vec<Context> = list.iter().map(action_context).collect();
    let mut context = Context::new();
    context.insert("actions", &actions);
    context
}

fn action_context(action:&InternalAction) -> Context{
    let mut context = Context::new();
    context.insert("name", &action.name);
    context.insert("source", &action.source);
    context.insert("id", &action.index);
    context.insert("type", print_action_type(&action.action_type));
    context.insert("css", print_css_class(&action.action_type) );
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

fn print_css_class(action: &ActionType) -> &'static str{
    match action {
        ActionType::Entertainment => "entertainment",
        ActionType::Task(Priority::JustForFun) => "fun",
        ActionType::Task(Priority::NiceToHave) => "nice",
        ActionType::Task(Priority::Useful) => "useful",
        ActionType::Task(Priority::Important) => "important",
        ActionType::Task(Priority::VeryImportant) => "very-important",
        ActionType::Task(Priority::Critical) => "critical",
        ActionType::Task(Priority::Mandatory) => "mandatory",
    }
}