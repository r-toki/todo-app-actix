use actix_web::{get, post, web, HttpResponse, Responder};

use super::domain::Task;
use super::dto::{TaskCreateRequestDto, TaskResponseDto};

#[get("/tasks")]
pub async fn index() -> impl Responder {
    let task1 = Task {
        id: "1".to_string(),
        description: "Buy a new gaming laptop".to_string(),
    };

    let task2 = Task {
        id: "2".to_string(),
        description: "Create video for YouTube".to_string(),
    };

    let res: Vec<TaskResponseDto> = vec![task1, task2]
        .iter()
        .map(|e| TaskResponseDto::from(e))
        .collect();

    HttpResponse::Ok().json(res)
}

#[post("/tasks")]
pub async fn create(body: web::Json<TaskCreateRequestDto>) -> impl Responder {
    let task = Task {
        id: "3".to_string(),
        description: body.description.clone(),
    };

    HttpResponse::Ok().json(TaskResponseDto::from(&task))
}
