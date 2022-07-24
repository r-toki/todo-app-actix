use actix_web::{get, HttpResponse, Responder};

use super::dto::TaskResponseDto;

#[get("/tasks")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(TaskResponseDto {
        id: "1".to_string(),
        description: "clean your room".to_string(),
    })
}
