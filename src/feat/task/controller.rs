use actix_web::{get, HttpResponse, Responder};

use super::domain::Task;
use super::dto::TaskResponseDto;

#[get("/tasks")]
pub async fn index() -> impl Responder {
    let task = Task {
        id: "1".to_string(),
        description: "clean your room".to_string(),
    };

    HttpResponse::Ok().json(TaskResponseDto::from(task))
}
