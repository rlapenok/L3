use std::sync::Arc;

use axum::async_trait;
use uuid::Uuid;

use crate::{
    domain::{
        models::{Claims, DeletePost, Post, User},
        social_network_repository::SocialNetworkRepository,
        social_network_service::SocialNetworkService,
        token_manager::TokenManager,
    },
    errors::ServerErrors,
};

#[derive(Clone)]
pub struct ServerState {
    repo: Arc<dyn SocialNetworkRepository + Send + Sync>,
    token_manager: Arc<dyn TokenManager + Send + Sync>,
}

impl ServerState {
    pub fn new(
        repo: Arc<dyn SocialNetworkRepository + Send + Sync>,
        token_manager: Arc<dyn TokenManager + Send + Sync>,
    ) -> Self {
        Self {
            repo,
            token_manager,
        }
    }
}

#[async_trait]
impl SocialNetworkService for ServerState {
    async fn register(&self, data: User) -> Result<(Uuid, String), ServerErrors> {
        let user_uid = data.user_uid;
        self.repo.register(data).await?;
        let token = self.token_manager.create_token(user_uid)?;
        Ok((user_uid, token))
    }
    async fn login(&self, data: User) -> Result<String, ServerErrors> {
        let user_uid=data.user_uid;
        self.repo.login(data).await?;
        self.token_manager.create_token(user_uid)
    }
    async fn create_post(&self, data: Post) -> Result<(), ServerErrors> {
        self.repo.create_post(data).await
    }
    async fn get_post(&self, post_uid: Uuid) -> Result<Post, ServerErrors> {
        self.repo.get_post(post_uid).await
    }
    async fn delete_post(&self,post: DeletePost)->Result<(),ServerErrors>{
        self.repo.delete_post(post).await
    }
    async fn like_post(&self,post_uid: Uuid)->Result<(),ServerErrors>{
        self.repo.like_post(post_uid).await
    }
    fn check_token(&self, token: &str)->Result<Claims, ServerErrors>{
        self.token_manager.check_token(token)
    }
}
