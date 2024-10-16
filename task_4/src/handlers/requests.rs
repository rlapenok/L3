use axum::{async_trait, extract::{rejection::JsonRejection, FromRequest, Request}, Json};
use serde::{de::DeserializeOwned, Deserialize};
use validator::Validate;

use crate::server_errors::ServerError;



pub struct JsonExtractor<T>(pub T);


#[async_trait]
impl <T,S> FromRequest<S> for JsonExtractor<T> 
    where S:Send+Sync,
        T:DeserializeOwned+Validate,
        Json<T>:FromRequest<S,Rejection = JsonRejection>
{
    type Rejection=ServerError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
            let data=Json::<T>::from_request(req, state).await?;
            data.0.validate()?;
            Ok(Self(data.0))
    }
    
}



#[derive(Deserialize,Validate)]
pub struct CreateUser{
    #[validate(length(min=1))]
    name:String,
    #[validate(email)]
    email:String,
}

#[derive(Deserialize,Validate,Debug)]
pub struct Price{
    #[validate(range(min=1))]
    rubles:usize,
    #[validate(range(max=99))]
    kopecs:usize,
}


#[derive(Deserialize,Validate,Debug)]
pub struct CreateProduct{
    #[validate(length(min=1))]
    name:String,
    #[validate(nested)]
    price:Price
}
