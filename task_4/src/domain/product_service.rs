use axum::async_trait;
use uuid::Uuid;

use crate::server_errors::ServerError;

use super::products_models::{Product, UpdateProduct};

#[async_trait]
pub trait ProductService {
    async fn create_product(&self, product: Product) -> Result<(), ServerError>;
    async fn update_product(&self, product: UpdateProduct) -> Result<(), ServerError>;
    async fn delete_product(&self, id: Uuid) -> Result<(), ServerError>;
}
