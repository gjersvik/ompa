use models::Action;
use tera::{Context, Tera};

/// Load in templates from the project template folder.
lazy_static! {
    static ref TERA: Tera =
        compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
}

pub fn view_list(list: &Vec<Action>) -> String{
    let mut context = Context::new();
    context.insert("actions", list);
    TERA.render("index.html", &context).unwrap()
}