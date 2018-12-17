use ompa::Config;
use std::env;

fn main() {
    dotenv::dotenv().ok();

    let config = Config {
        bind_port: env::var("PORT").expect("PORT not found"),
        web_password: env::var("OMPA_PASSWORD").expect("OMPA_PASSWORD not found"),
        postgresql_uri: env::var("POSTGRESQL_URI").expect("POSTGRESQL_URI not found"),
    };

    ompa::start(config);
}
