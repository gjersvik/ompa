#![deny(clippy::all)]

mod housework;
mod ole_martin;
mod test_actions;

use actix::{Actor, System};

use crate::{
    test_actions::TestActions,
    housework::Chores,
};

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn start() {
    let sys = System::new("ompa");

    let _test = TestActions::start_default();
    let _chore = Chores::start_default();

    sys.run();
}