use actix_web::{middleware::Logger, App, HttpServer};

mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(move || App::new().wrap(Logger::default()).service(user::info))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
