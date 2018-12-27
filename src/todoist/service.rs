use super::messages::{Sync, SyncResult};
use actix::{Actor, Handler, SyncContext};
use todoist::{Client, ResourceType};

pub struct Service {
    token: String,
}

impl Service {
    pub fn new(token: String) -> Service {
        Service { token }
    }
}

impl Actor for Service {
    type Context = SyncContext<Self>;
}

impl Handler<Sync> for Service {
    type Result = SyncResult;

    fn handle(&mut self, msg: Sync, _: &mut Self::Context) -> Self::Result {
        let mut client = match msg.0 {
            Some(token) => Client::new_with_sync(&self.token, &token),
            None => Client::new(&self.token),
        };
        
        match client.sync(&[ResourceType::Items]) {
            Ok(res) => SyncResult::from_response(res),
            Err(e) => {
                eprintln!("Todoist sync error: {}", e);
                SyncResult::empty()
            },
        }
    }
}
