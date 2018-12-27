use actix::{
    dev::{MessageResponse, ResponseChannel},
    Actor, Message,
};
use todoist::{resource::Item,
SyncResponse,
};

pub struct SyncResult{
    pub sync_token: Option<String>,
    pub items: Vec<Item>,
}

impl SyncResult {
    pub fn empty() -> SyncResult{
        SyncResult {
            sync_token: None,
            items: Vec::new(),
        }
    }

    pub fn from_response(res: SyncResponse) -> SyncResult{
        SyncResult {
            sync_token: Some(res.sync_token),
            items: res.items.unwrap_or_default(),
        }
    }
}

impl<A, M> MessageResponse<A, M> for SyncResult
where
    A: Actor,
    M: Message<Result = SyncResult>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

pub struct Sync(pub Option<String>);

impl Message for Sync {
    type Result = SyncResult;
}

#[derive(Message)]
pub struct Complete(pub usize);
