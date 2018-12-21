use actix::{
    dev::{MessageResponse, ResponseChannel},
    Actor, Message,
};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Chore {
    pub id: i32,
    pub name: String,
    pub frequency: Duration,
    pub last_done: Option<DateTime<Utc>>,
}

#[derive(Clone, Default)]
pub struct Chores(pub HashMap<i32, Chore>);

impl<A, M> MessageResponse<A, M> for Chores
where
    A: Actor,
    M: Message<Result = Chores>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

pub struct LoadChores;

impl Message for LoadChores {
    type Result = Chores;
}

#[derive(Message)]
pub struct UpdateTime(pub i32, pub DateTime<Utc>);
