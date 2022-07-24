use actix_web::{get, post, web, HttpResponse, Responder};

use super::domain::Task;
use super::dto::{TaskCreateRequestDto, TaskResponseDto};

#[get("/tasks")]
pub async fn index() -> impl Responder {
    let task = Task {
        id: "1".to_string(),
        description: "clean your room".to_string(),
    };

    HttpResponse::Ok().json(TaskResponseDto::from(task))
}

#[post("/tasks")]
pub async fn create(body: web::Json<TaskCreateRequestDto>) -> impl Responder {
    let task = Task {
        id: "2".to_string(),
        description: body.description.to_string(),
    };

    HttpResponse::Ok().json(TaskResponseDto::from(task))
}
