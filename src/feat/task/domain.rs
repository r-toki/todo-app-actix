#[derive(Clone)]
pub struct Task {
    pub id: String,
    pub description: String,
}

pub trait TaskRepository {
    fn save(&self, task: Task);
    fn all(&self) -> Vec<Task>;
}
