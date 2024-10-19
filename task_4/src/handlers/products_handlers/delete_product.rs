use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{domain::product_service::ProductService, server_errors::ServerError};

pub async fn delete_product<S>(
    State(state): State<S>,
    Path(product_id): Path<Uuid>,
) -> Result<(), ServerError>
where
    S: ProductService,
{
    state.delete_product(product_id).await?;
    Ok(())
}
