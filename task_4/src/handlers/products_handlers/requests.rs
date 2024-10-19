use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Deserialize,Validate)]
pub struct UpdatePrice{
    pub rubles:Option<i64>,
    #[validate(custom(function = "kopecs_validation"))]
    pub kopecs:Option<i64>
}

fn kopecs_validation(data:i64)->Result<(),ValidationError>{
        if data>99{
            return  Err(ValidationError::new("more than 99"))
        }
        return Ok(())
    }


#[derive(Deserialize,Validate)]
pub struct UpdateProductRequest{
    pub name:Option<String>,
    #[validate(nested)]
    pub price:Option<UpdatePrice>
}


#[derive(Deserialize, Validate, Debug)]
pub struct Price {
    #[validate(range(min = 1))]
    pub rubles: i64,
    #[validate(range(max = 99))]
    pub kopecs: i64,
}

#[derive(Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(nested)]
    pub price: Price,
}