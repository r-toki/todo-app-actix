use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug)]
pub struct NewTask {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub done: bool,
}

impl Task {
    pub async fn all(pool: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
SELECT id, description, done
FROM tasks
ORDER BY id            
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }

    pub async fn insert(new_task: NewTask, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
INSERT INTO tasks (description)
VALUES ( $1 )          
            "#,
            new_task.description
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
