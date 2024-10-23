use uuid::Uuid;

use crate::handlers::products_handlers::requests::CreateProductRequest;

pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub rubles: i64,
    pub kopecs: i64,
}

impl From<CreateProductRequest> for Product {
    fn from(value: CreateProductRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: value.name,
            rubles: value.price.rubles,
            kopecs: value.price.kopecs,
        }
    }
}

pub struct UpdateProduct {
    pub id: Uuid,
    pub name: Option<String>,
    pub rubles: Option<i64>,
    pub kopecs: Option<i64>,
}

impl UpdateProduct {
    pub fn new(id: Uuid, name: Option<String>, rubles: Option<i64>, kopecs: Option<i64>) -> Self {
        Self {
            id,
            name,
            rubles,
            kopecs,
        }
    }
}
