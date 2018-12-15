#![deny(clippy::all)]

mod housework;
mod logger;
mod ole_martin;
mod test_actions;

use actix::{Actor, System};
use actix_web::server;

use crate::{
    test_actions::TestActions,
    housework::Chores,
    ole_martin::OleMartin,
    logger::Logger,
};

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn start() {
    let sys = System::new("ompa");

    let _test = TestActions::start_default();
    let _chore = Chores::start_default();
    let _log = Logger::addr();

    let server = server::new(OleMartin::app);
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    let server = server.bind(addr).unwrap();
    server.start();

    sys.run();
}