use chrono::{Duration, Local};
use jsonwebtoken::{
    decode, encode, Algorithm::HS512, DecodingKey, EncodingKey, Header, Validation
};
use uuid::Uuid;

use crate::{
    domain::{models::Claims, token_manager::TokenManager},
    errors::ServerErrors,
};

pub struct SecretVault {
    secret: String,
    exp_time: Duration,
    validator: Validation,
}

impl SecretVault {
    pub fn new(secret: String, exp_time: i64) -> Self {
        let exp_time = Duration::minutes(exp_time);
        let mut validator = Validation::new(HS512);
        validator.set_required_spec_claims(&["sub","iat","exp"]);
        validator.validate_aud = false;
        Self {
            secret,
            exp_time,
            validator,
        }
    }
}

impl SecretVault {
    fn create_header(&self) -> Header {
        Header::new(HS512)
    }
}

impl TokenManager for SecretVault {
    fn create_token(&self, user_uid: Uuid) -> Result<String, ServerErrors> {
        let header = self.create_header();
        let now = Local::now();
        let iat = Local::now().timestamp() as usize;
        let exp = (now + self.exp_time).timestamp() as usize;
        let claims = Claims::new(user_uid, iat, exp);
        let key = EncodingKey::from_secret(self.secret.as_ref());
        let token = encode(&header, &claims, &key).map_err(|err|{
            ServerErrors::CreateTokenError(err.to_string())
        })?;
        Ok(token)
    }
    fn check_token(&self, token: &str) -> Result<Claims, ServerErrors> {
        let result=decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &self.validator,
        ).map_err(|err|{
            ServerErrors::ParseTokenError(err.to_string())
        })?;
        Ok(result.claims)
    }
}
