use serde::Serialize;

#[derive(Serialize)]
pub struct TaskResponseDto {
    pub id: String,
    pub description: String,
}
