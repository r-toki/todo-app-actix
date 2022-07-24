use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

use super::model::{NewTask, Task};

#[get("/tasks")]
pub async fn index(pool: web::Data<PgPool>) -> impl Responder {
    let res = Task::all(&pool).await.unwrap();

    HttpResponse::Ok().json(res)
}

#[derive(Deserialize)]
pub struct TaskCreateRequestDto {
    description: String,
}

#[post("/tasks")]
pub async fn create(
    dto: web::Json<TaskCreateRequestDto>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let new_task = NewTask {
        description: dto.description.clone(),
    };

    Task::insert(new_task, &pool).await.unwrap();

    HttpResponse::Ok()
}
