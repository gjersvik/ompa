use ompa::Config;
use std::env;

fn main() {
    dotenv::dotenv().ok();

    let config = Config {
        bind_port: env::var("PORT").expect("PORT not found"),
        web_password: env::var("OMPA_PASSWORD").expect("OMPA_PASSWORD not found"),
        postgresql_uri: env::var("DATABASE_URL").expect("DATABASE_URL not found"),
        todoist_token: env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN not found"),
    };

    ompa::start(config);
}
