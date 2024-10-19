use axum::async_trait;
use uuid::Uuid;

use crate::infrastructe::errors::RepoError;

use super::{
    products_models::{Product, UpdateProduct}, utils::{CloseRepository, ToChangeNotifier}
};

#[async_trait]
pub trait ProductRepository: ToChangeNotifier + CloseRepository {
    async fn create_product(&self, product: Product) -> Result<(), RepoError>;
    async fn update_product(&self, user: UpdateProduct) -> Result<(), RepoError>;
    async fn delete_product(&self, id: Uuid) -> Result<(), RepoError>;
}
