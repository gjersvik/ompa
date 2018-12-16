#![deny(clippy::all)]

mod housework;
mod logger;
mod ole_martin;
mod test_actions;

use actix::{Actor, System};
use actix_web::{
    server,
    Result,
    HttpRequest,
    FromRequest,
    middleware::{Middleware, Started},
};
use actix_web_httpauth::extractors::{
    AuthenticationError,
    basic::{BasicAuth, Config as AuthConfig},
};

use crate::{
    test_actions::TestActions,
    housework::Chores,
    ole_martin::OleMartin,
    logger::Logger,
};

#[derive(Clone)]
struct Auth{
    password: String,
}

impl Auth {
    fn new(password: String) -> Auth{
        Auth {password}
    }
}

impl<S> Middleware<S> for Auth {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let mut config = AuthConfig::default();
        config.realm("Ompa");
        let auth = BasicAuth::from_request(&req, &config)?;

        if auth.username() == "olem" && auth.password() == Some(&self.password) {
            Ok(Started::Done)
        } else {
            Err(AuthenticationError::from(config).into())
        }
    }
}

pub struct Config{
    pub bind_port: String,
    pub web_password: String,
}

pub fn start(config: Config) {
    let sys = System::new("ompa");

    let _test = TestActions::start_default();
    let _chore = Chores::start_default();
    let _log = Logger::addr();

    let auth = Auth::new(config.web_password);

    let server = server::new(move || {
        OleMartin::app().middleware(auth.clone())
    });
    let addr = "0.0.0.0:".to_string() + &config.bind_port;
    println!("Listening for requests at http://{}", addr);
    let server = server.bind(addr).unwrap();
    server.start();

    sys.run();
}