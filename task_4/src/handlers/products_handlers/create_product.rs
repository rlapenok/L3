use axum::extract::State;

use crate::{domain::{product_service::ProductService, products_models::Product}, handlers::{json_extractor_with_validation::JsonExtractor, reponses::CreateUserProductResponse}, server_errors::ServerError};

use super::requests::CreateProductRequest;




pub async fn create_product<S>(State(state):State<S>,
    JsonExtractor(req): JsonExtractor<CreateProductRequest>,
) -> Result<CreateUserProductResponse, ServerError> 
    where S:ProductService
{
    let product=Product::from(req);
    let id=product.id;
    state.create_product(product).await?;
    Ok(CreateUserProductResponse{
        id
    })
}
