use serde::{Deserialize, Serialize};

use super::domain::Task;

#[derive(Serialize)]
pub struct TaskResponseDto {
    pub id: String,
    pub description: String,
}

impl TaskResponseDto {
    pub fn from(task: Task) -> TaskResponseDto {
        TaskResponseDto {
            id: task.id,
            description: task.description,
        }
    }
}

#[derive(Deserialize)]
pub struct TaskCreateRequestDto {
    pub description: String,
}
