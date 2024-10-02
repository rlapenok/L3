use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::{
    api::request_structs::{LoginRequest, RegisterRequest},
    errors::ServerErrors,
};

#[derive(Serialize, Deserialize,Debug)]
pub struct Claims {
    pub sub: Uuid,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn new(user_uid: Uuid, iat: usize, exp: usize) -> Self {
        Self { sub: user_uid, iat, exp }
    }
}

pub struct User {
    pub user_uid: Uuid,
    pub login: String,
    pub hashed_password: String,
}

impl From<RegisterRequest> for User {
    fn from(value: RegisterRequest) -> Self {
        let user_uid = Uuid::new_v4();
        Self {
            user_uid,
            login: value.login,
            hashed_password: value.hashed_password,
        }
    }
}

impl From<LoginRequest> for User {
    fn from(value: LoginRequest) -> Self {
        User {
            user_uid: value.user_uid,
            login: value.login,
            hashed_password: value.hashed_password,
        }
    }
}


#[derive(Serialize)]
pub struct Post {
    pub user_uid: Uuid,
    pub post_uid: Uuid,
    pub msg: String,
    pub likes: i64,
}
impl Post{
    pub fn new(user_uid:Uuid,msg:String)->Self{
        Self { user_uid, post_uid: Uuid::new_v4(), msg, likes:0 }
    }
}

impl TryFrom<Row> for Post {
    type Error = ServerErrors;
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            user_uid: value.try_get("user_uid")?,
            post_uid: value.try_get("post_uid")?,
            msg: value.try_get("msg")?,
            likes: value.try_get("likes")?,
        })
    }
}

impl IntoResponse for Post {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug)]
pub struct DeletePost{
    pub user_uid:Uuid,
    pub post_uid:Uuid,
}
