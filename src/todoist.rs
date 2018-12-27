mod messages;
mod service;

use self::{
    service::Service,
    messages::Sync,
};
use actix::{SyncArbiter,Arbiter};
use futures::future::Future;

pub fn todoist(token: String){
    let service = SyncArbiter::start(1, move|| Service::new(token.clone()));

    let run = service.send(Sync(None)).map(|res| {
        for item in res.items {
            println!("{}", item.content.unwrap_or_default())
        };
    }).map_err(|_|{});

    Arbiter::spawn(run);
}