use axum::async_trait;
use uuid::Uuid;

use crate::errors::ServerErrors;

use super::models::{DeletePost, Post,  User};

#[async_trait]
pub trait SocialNetworkRepository {
    async fn register(&self, data: User) -> Result<(), ServerErrors>;
    async fn login(&self, data: User) -> Result<(), ServerErrors>;
    async fn create_post(&self, data: Post) -> Result<(), ServerErrors>;
    async fn get_post(&self, post_uid: Uuid) -> Result<Post, ServerErrors>;
    async fn delete_post(&self,post:DeletePost)->Result<(),ServerErrors>;
    async fn like_post(&self,post_uid: Uuid)->Result<(),ServerErrors>;
}
