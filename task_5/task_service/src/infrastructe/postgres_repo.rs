use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    models::{NewTask, Task},
    task_repository::TaskRepository,
};

use super::table_listener::TableListener;
#[derive(Clone)]

pub struct PostgresRepo(PgPool);
impl PostgresRepo {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl TaskRepository for PostgresRepo {
    type ChangeListener=TableListener;
    async fn create_task(&self, task: NewTask) -> Result<(), sqlx::Error> {
        let mut tx = self.0.begin().await?;
        sqlx::query("INSERT INTO tasks (id,description) VALUES ($1, $2)")
            .bind(task.id)
            .bind(task.description)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
    async fn get_task(&self, task_id: Uuid) -> Result<Task, sqlx::Error> {
        let mut tx = self.0.begin().await?;
        let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
            .bind(task_id)
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(task)
    }
    async fn complete_task(&self, task_id: Uuid) -> Result<(), sqlx::Error> {
        let mut tx = self.0.begin().await?;
        sqlx::query("UPDATE tasks SET completed = TRUE WHERE id = $1")
            .bind(task_id)
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
    async fn to_change_listener(&self)->Self::ChangeListener{
        todo!()
    }
    async fn close(&self){
        self.0.close().await;
    }
    
}
