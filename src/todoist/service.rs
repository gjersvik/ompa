use super::messages::{Complete, Sync, SyncResult};
use actix::{Actor, Handler, SyncContext};
use todoist::{Client, ResourceType, Error, CommandResponse};
use uuid::Uuid;
use reqwest::{Client as HttpClient};

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
            }
        }
    }
}

impl Handler<Complete> for Service {
    type Result = ();

    fn handle(&mut self, msg: Complete, _: &mut Self::Context) -> Self::Result {
        match send(&self.token, complete_command(msg.0)) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Todoist send error: {}", e);
            }
        }
    }
}

fn complete_command(id: usize) -> String {
    format!("[{{\"type\": \"item_close\", \"uuid\": \"{}\", \"args\": {{\"id\": {}}}}}]", Uuid::new_v4(), id)
}

fn send(token: &str, cmd: String) -> Result<CommandResponse, Error> {
    let client = HttpClient::new();
    let res : CommandResponse = client.post("http://todoist.com/api/v7/sync")
        .form(&[("token", token.to_string()), 
                ("commands", cmd)])
        .send()?
        .json()?;

    Ok(res)
}