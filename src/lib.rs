#![deny(clippy::all)]

mod chores;
mod logger;
mod ole_martin;

use actix::{Actor, System};
use actix_web::{
    middleware::{Middleware, Started},
    server, FromRequest, HttpRequest, Result,
};
use actix_web_httpauth::extractors::{
    basic::{BasicAuth, Config as AuthConfig},
    AuthenticationError,
};

use crate::{chores::Chores, logger::Logger, ole_martin::OleMartin};

#[derive(Clone)]
struct Auth {
    password: String,
}

impl Auth {
    fn new(password: String) -> Auth {
        Auth { password }
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

pub struct Config {
    pub bind_port: String,
    pub web_password: String,
    pub postgresql_uri: String,
}

pub fn start(config: Config) {
    let sys = System::new("ompa");

    let _chore = Chores::new(
        config.postgresql_uri,
        OleMartin::addr().recipient(),
        OleMartin::addr().recipient(),
    )
    .start();
    let _log = Logger::addr();

    let auth = Auth::new(config.web_password);

    let server = server::new(move || OleMartin::app().middleware(auth.clone()));
    let addr = "0.0.0.0:".to_string() + &config.bind_port;
    println!("Listening for requests at http://{}", addr);
    let server = server.bind(addr).unwrap();
    server.start();

    sys.run();
}
