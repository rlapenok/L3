use axum::async_trait;
use uuid::Uuid;

use crate::infrastructe::errors::RepoError;

use super::{
    users_models::{UpdateUser, User},
    utils::{CloseRepository, ToChangeNotifier},
};

#[async_trait]
pub trait UserRepository: ToChangeNotifier + CloseRepository {
    async fn create_user(&self, user: User) -> Result<(), RepoError>;
    async fn update_user(&self, user: UpdateUser) -> Result<(), RepoError>;
    async fn delete_user(&self, id: Uuid) -> Result<(), RepoError>;
}
