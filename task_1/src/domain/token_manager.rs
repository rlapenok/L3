use uuid::Uuid;

use crate::errors::ServerErrors;

use super::models::Claims;

pub trait TokenManager {
    fn create_token(&self, user_uid: Uuid) -> Result<String, ServerErrors>;
    fn check_token(&self, token: &str) -> Result<Claims, ServerErrors>;
}
