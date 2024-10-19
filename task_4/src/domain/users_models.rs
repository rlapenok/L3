use uuid::Uuid;

use crate::handlers::user_handlers::requests::CreateUserRequest;



pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl From<CreateUserRequest> for User {
    fn from(value: CreateUserRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: value.name,
            email: value.email,
        }
    }
}

pub struct UpdateUser {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl UpdateUser {
    pub fn new(id: Uuid, name: Option<String>, email: Option<String>) -> Self {
        Self { id, name, email }
    }
}
