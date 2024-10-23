use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{
    domain::{product_service::ProductService, products_models::UpdateProduct},
    handlers::json_extractor_with_validation::JsonExtractor,
    server_errors::ServerError,
};

use super::requests::UpdateProductRequest;

pub async fn update_product<S>(
    State(state): State<S>,
    Path(product_id): Path<Uuid>,
    JsonExtractor(req): JsonExtractor<UpdateProductRequest>,
) -> Result<(), ServerError>
where
    S: ProductService,
{
    let product = if let Some(price) = req.price {
        UpdateProduct::new(product_id, req.name, price.rubles, price.kopecs)
    } else {
        UpdateProduct::new(product_id, req.name, None, None)
    };
    state.update_product(product).await?;
    Ok(())
}
