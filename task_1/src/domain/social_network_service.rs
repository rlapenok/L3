use axum::async_trait;
use uuid::Uuid;

use crate::errors::ServerErrors;

use super::models::{Claims, DeletePost, Post, User};

#[async_trait]
pub trait SocialNetworkService {
    async fn register(&self, data: User) -> Result<(Uuid, String), ServerErrors>;
    async fn login(&self, data: User) -> Result<String, ServerErrors>;
    async fn create_post(&self, data: Post) -> Result<(), ServerErrors>;
    async fn get_post(&self, post_uid: Uuid) -> Result<Post, ServerErrors>;
    async fn delete_post(&self,post: DeletePost)->Result<(),ServerErrors>;
    async fn like_post(&self,post_uid: Uuid)->Result<(),ServerErrors>;
    fn check_token(&self, token: &str)->Result<Claims,ServerErrors>;
}
