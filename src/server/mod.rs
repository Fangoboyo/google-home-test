mod endpoints;

pub mod api_server {
    use rocket::{Build, Config, Rocket};
    use crate::server::endpoints::exit_handler;
    use crate::server::endpoints::enter_handler;

    pub fn app() -> Rocket<Build> {
        let config = Config {
            address: "0.0.0.0".to_string().parse().unwrap(),
            port: 3000,
            ..Config::default()
        };

        rocket::custom(config).mount("/api", routes![exit_handler, enter_handler])
    }
}