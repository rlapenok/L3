use crate::server_errors::ServerError;

use super::requests::{CreateProduct, JsonExtractor};

pub async fn create_product(JsonExtractor(req):JsonExtractor<CreateProduct>)->Result<(),ServerError>{
    println!("{:?}",req);
    Ok(())
}