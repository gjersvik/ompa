use actix::{Actor, Addr, Context, Handler, Supervised, System, SystemService};
use std::collections::HashMap;
use rand::{
    thread_rng,
    Rng,
    seq::SliceRandom
};

use super::{
    messages::{Actions, GetAction, GetActions, InternalAction, ActionType, Priority},
    Action, UpdateActions,
};

#[derive(Default)]
pub struct Sorter {
    sources: HashMap<String, Vec<Action>>,
}

impl Sorter {
    pub fn addr() -> Addr<Sorter> {
        System::current().registry().get::<Sorter>()
    }
}

impl Actor for Sorter {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {}
}

impl Supervised for Sorter {}

impl SystemService for Sorter {
    fn service_started(&mut self, _: &mut Context<Self>) {}
}

impl Handler<UpdateActions> for Sorter {
    type Result = ();

    fn handle(&mut self, msg: UpdateActions, _: &mut Self::Context) {
        self.sources.insert(msg.name, msg.actions);
    }
}

impl Handler<GetActions> for Sorter {
    type Result = Actions;

    fn handle(&mut self, _: GetActions, _: &mut Self::Context) -> Self::Result {
        // Separate out entertainment.
        let (mut entertains, tasks): (Vec<InternalAction>, Vec<InternalAction>) = self.sources.iter().flat_map(to_internal).partition(is_entertainment);

        // shuffle entertainment.
        entertains.shuffle(&mut thread_rng());

        // Score tasks. 
        let mut scored: Vec<(&InternalAction, f64)> = tasks.iter().map(score).collect();

        // Sort based on score.
        scored.sort_unstable_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
        let tasks = scored.iter().map(|s| s.0).cloned().collect();

        // Merge tasks and entertainment
        Actions(merge(tasks, entertains))
    }
}

impl Handler<GetAction> for Sorter {
    type Result = Option<InternalAction>;

    fn handle(&mut self, msg: GetAction, _: &mut Self::Context) -> Self::Result {
        let actions = self.sources.get(&msg.0)?;
        let action = actions.iter().find(|a| a.index == msg.1)?;

        Some(InternalAction::new(action.clone(), msg.0))
    }
}

fn to_internal(kv: (&String, &Vec<Action>)) -> Vec<InternalAction> {
    let (name, actions) = kv;
    actions
        .iter()
        .map(|a| InternalAction::new(a.clone(), name.clone()))
        .collect()
}

fn is_entertainment(a: &InternalAction) -> bool {
    match &a.action_type{
        ActionType::Entertainment => true,
        _ => false,
    }
}

fn score(a: &InternalAction) -> (&InternalAction, f64) {
    let rnd = thread_rng().gen();

    let score = match &a.action_type{
        ActionType::Entertainment => 0.0,
        ActionType::Task(Priority::Mandatory) => 2.0,
        ActionType::Task(Priority::Critical) => 1.0,
        ActionType::Task(Priority::VeryImportant) => (rnd / 10000.0) + 0.9999,
        ActionType::Task(Priority::Important) => (rnd / 1000.0) + 0.999,
        ActionType::Task(Priority::Useful) => (rnd / 100.0) + 0.99,
        ActionType::Task(Priority::NiceToHave) => (rnd / 10.0) + 0.9,
        ActionType::Task(Priority::JustForFun) => rnd,
    };

    (a, score)
}

fn merge(mut task: Vec<InternalAction>,mut entertainment: Vec<InternalAction>) -> Vec<InternalAction> {
    let mut out = Vec::new();

    loop{
        if task.len() >= 3{
            let temp = task.split_off(3);
            out.append(&mut task);
            task = temp;
        }

        if !entertainment.is_empty(){
            out.push(entertainment.remove(0))
        }else{
            out.append(&mut task);
            break;
        }
    }

    out
}
