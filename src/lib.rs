#![deny(clippy::all)]

mod housework;
mod models;
mod ole_martin;
mod test_actions;
mod views;

use crate::ole_martin::UpdateActions;
use crate::ole_martin::OleMartin;
use futures::future::Future;
use actix::{Arbiter, System};

use crate::{
    models::GetActions,
    test_actions::TestActions,
    housework::Chores,
};

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn start() {
    let sys = System::new("ompa");

    let test = Arbiter::registry().get::<TestActions>();
    let _chore = System::current().registry().get::<Chores>();
    let tests = test.send(GetActions{}).and_then(|actions| System::current().registry().get::<OleMartin>().send(UpdateActions{name: "test".to_string(), actions: actions.actions}));
    Arbiter::spawn(tests.map(|_|{}).map_err(|_|{}));

    sys.run();
}