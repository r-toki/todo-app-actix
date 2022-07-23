mod controller;

use actix_web::{middleware::Logger, App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // NOTE: $ heroku config:set HOST=0.0.0.0
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(controller::tasks::index)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
