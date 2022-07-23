use actix_web::{get, Responder};

#[get("/tasks")]
pub async fn index() -> impl Responder {
    "HELLO WORLD!"
}
