use std::{error::Error, sync::Arc};

use axum::async_trait;
use tokio::signal::ctrl_c;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use uuid::Uuid;

use crate::{
    domain::{
        change_notifier::TableChangeNotifier, product_repository::ProductRepository, product_service::ProductService, products_models::{Product, UpdateProduct}, user_repository::UserRepository, user_service::UserService, users_models::{UpdateUser, User}, utils::CloseRepository
    },
    server_errors::ServerError,
};

#[derive(Clone)]
pub struct ServerState<T, N>
{
    repo: T,
    notifier: N,
    background_task_tracker: TaskTracker,
    background_cancellation_token: Arc<CancellationToken>,
}

impl<T, N> ServerState<T, N>
where
    T: UserRepository + CloseRepository + Clone + Sync,
    N: TableChangeNotifier + Clone + Sync,
{
    pub fn new(repo: T, notifier: N, background_cancellation_token: CancellationToken) -> Self {
        Self {
            repo,
            notifier,
            background_task_tracker: TaskTracker::new(),
            background_cancellation_token: Arc::new(background_cancellation_token),
        }
    }
    pub async fn shutdown(&self) -> Result<(), Box<dyn Error>> {
        self.background_task_tracker.close();
        self.background_cancellation_token.cancel();
        self.background_task_tracker.wait().await;
        self.notifier.stop_notifier().await?;
        self.repo.close().await;
        Ok(())
    }
}

#[async_trait]
impl<T, N> UserService for ServerState<T, N>
where
    T: UserRepository + CloseRepository + Clone + Sync,
    N: TableChangeNotifier + Clone + Sync,
{
    async fn create_user(&self, user: User) -> Result<(), ServerError> {
        self.repo.create_user(user).await?;
        Ok(())
    }
    async fn update_user(&self, user: UpdateUser) -> Result<(), ServerError> {
        self.repo.update_user(user).await?;
        Ok(())
    }
    async fn delete_user(&self, id: Uuid) -> Result<(), ServerError> {
        self.repo.delete_user(id).await?;
        Ok(())
    }
}

#[async_trait]
impl<T, N> ProductService for ServerState<T, N>
where
    T: ProductRepository + CloseRepository + Clone + Sync,
    N: TableChangeNotifier + Clone + Sync,
{
    async fn create_product(&self, product: Product) -> Result<(), ServerError> {
        self.repo.create_product(product).await?;
        Ok(())
    }
    async fn update_product(&self, product: UpdateProduct) -> Result<(), ServerError> {
        self.repo.update_product(product).await?;
        Ok(())
    }
    async fn delete_product(&self, id: Uuid) -> Result<(), ServerError> {
        self.repo.delete_product(id).await?;
        Ok(())
    }
}



pub async fn gracefull_shutdown<T, N>(state: ServerState<T, N>)
where
    T: UserRepository + CloseRepository + Clone + Sync,
    N: TableChangeNotifier + Clone + Sync,
{
    ctrl_c().await.expect("failed to install Ctrl+C handler");
    state.shutdown().await.expect("Error while shutdown server");
}
