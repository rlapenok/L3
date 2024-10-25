use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::api::requests::CreateTaskRequest;

pub struct NewTask {
    pub id: Uuid,
    pub description: String,
}

impl From<CreateTaskRequest> for NewTask {
    fn from(value: CreateTaskRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: value.description,
        }
    }
}

#[derive(Serialize, FromRow)]
pub struct Task {
    id: Uuid,
    description: String,
    completed: bool,
}
