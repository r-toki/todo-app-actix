use actix_web::{middleware::Logger, App, HttpServer};
use std::env;

mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap();

    HttpServer::new(move || App::new().wrap(Logger::default()).service(user::info))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
