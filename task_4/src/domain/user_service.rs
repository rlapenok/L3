use axum::async_trait;
use uuid::Uuid;

use crate::server_errors::ServerError;

use super::users_models::{UpdateUser, User};

#[async_trait]
pub trait UserService {
    async fn create_user(&self, user: User) -> Result<(), ServerError>;
    async fn update_user(&self, user: UpdateUser) -> Result<(), ServerError>;
    async fn delete_user(&self, id: Uuid) -> Result<(), ServerError>;
}
