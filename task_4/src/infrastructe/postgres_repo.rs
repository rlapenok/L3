use axum::async_trait;
use log::error;
use sqlx::{postgres::PgListener, ConnectOptions, PgPool};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::domain::{
    change_notifier::TableChanges,
    product_repository::ProductRepository,
    products_models::{Product, UpdateProduct},
    user_repository::UserRepository,
    users_models::{UpdateUser, User},
    utils::{CloseRepository, ToChangeNotifier},
};

use super::{
    errors::{RepoError, RepoType},
    notifier::Notifier,
};

#[derive(Clone)]
pub struct PostgresRepo(pub(crate) PgPool);
impl PostgresRepo {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl UserRepository for PostgresRepo {
    async fn create_user(&self, user: User) -> Result<(), RepoError> {
        let mut tx = self
            .0
            .begin()
            .await
            .map_err(|err| RepoError::from(err, RepoType::Users)).inspect_err(|err|{
                error!("{}",err);
            })?;
        sqlx::query("INSERT INTO users (id,name,email) VALUES ($1, $2, $3)")
            .bind(user.id)
            .bind(user.name)
            .bind(user.email)
            .execute(&mut *tx)
            .await
            .map_err(|err| RepoError::from(err, RepoType::Users)).inspect_err(|err|{
                error!("{}",err);
            })?;
        tx.commit()
            .await
            .map_err(|err| RepoError::from(err, RepoType::Users)).inspect_err(|err|{
                error!("{}",err);
            })?;
        Ok(())
    }
    async fn update_user(&self, user: UpdateUser) -> Result<(), RepoError> {
        let mut tx = self
            .0
            .begin()
            .await
            .map_err(|err| RepoError::from(err, RepoType::Users)).inspect_err(|err|{
                error!("{}",err);
            })?;
        sqlx::query(
            "UPDATE users SET name=COALESCE($1,name),email=COALESCE($2,email)
                    WHERE id=$3
        ",
        )
        .bind(user.name)
        .bind(user.email)
        .bind(user.id)
        .execute(&mut *tx)
        .await
        .map_or_else(
            |err| Err(RepoError::from(err, RepoType::Users)),
            |result| {
                if result.rows_affected() != 1 {
                    return Err(RepoError::IdNotExist);
                }
                Ok(())
            },
        ).inspect_err(|err|{
            error!("{}",err);
        })?;
        tx.commit()
            .await
            .map_err(|err| RepoError::from(err, RepoType::Users)).inspect_err(|err|{
                error!("{}",err);
            })?;
        Ok(())
    }
    async fn delete_user(&self, id: Uuid) -> Result<(), RepoError> {
        let mut tx = self
            .0
            .begin()
            .await
            .map_err(|err| RepoError::from(err, RepoType::Users)).inspect_err(|err|{
                error!("{}",err);
            })?;
        sqlx::query("DELETE FROM users WHERE id=$1")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_or_else(
                |err| Err(RepoError::from(err, RepoType::Users)),
                |result| {
                    if result.rows_affected() != 1 {
                        return Err(RepoError::IdNotExist);
                    }
                    Ok(())
                },
            ).inspect_err(|err|{
                error!("{}",err);
            })?;

        tx.commit()
            .await
            .map_err(|err| RepoError::from(err, RepoType::Users)).inspect_err(|err|{
                error!("{}",err);
            })?;
        Ok(())
    }
}

#[async_trait]
impl ProductRepository for PostgresRepo {
    async fn create_product(&self, product: Product) -> Result<(), RepoError> {
        let mut tx = self
            .0
            .begin()
            .await
            .map_err(|err| RepoError::from(err, RepoType::Products)).inspect_err(|err|{
                error!("{}",err);
            })?;
        sqlx::query("INSERT INTO products (id,name,rubles,kopecs) VALUES ($1, $2, $3, $4)")
            .bind(product.id)
            .bind(product.name)
            .bind(product.rubles)
            .bind(product.kopecs)
            .execute(&mut *tx)
            .await
            .map_err(|err| RepoError::from(err, RepoType::Products)).inspect_err(|err|{
                error!("{}",err);
            })?;
        tx.commit()
            .await
            .map_err(|err| RepoError::from(err, RepoType::Products)).inspect_err(|err|{
                error!("{}",err);
            })?;
        Ok(())
    }
    async fn update_product(&self, product: UpdateProduct) -> Result<(), RepoError> {
        let mut tx = self
            .0
            .begin()
            .await
            .map_err(|err| RepoError::from(err, RepoType::Products)).inspect_err(|err|{
                error!("{}",err);
            })?;
        sqlx::query(
            "UPDATE products SET name=COALESCE($1,name),rubles=COALESCE($2,rubles),kopecs=COALESCE($3,kopecs)
                    WHERE id=$4
        ",
        )
        .bind(product.name)
        .bind(product.rubles)
        .bind(product.kopecs)
        .bind(product.id)
        .execute(&mut *tx)
        .await
        .map_or_else(|err|{
            Err(RepoError::from(err, RepoType::Products))
        }, |result|{
            if result.rows_affected() != 1 {
                return Err(RepoError::IdNotExist);
            }
            Ok(())
        }).inspect_err(|err|{
            error!("{}",err);
        })?;
        tx.commit()
            .await
            .map_err(|err| RepoError::from(err, RepoType::Products)).inspect_err(|err|{
                error!("{}",err);
            })?;
        Ok(())
    }
    async fn delete_product(&self, id: Uuid) -> Result<(), RepoError> {
        let mut tx = self
            .0
            .begin()
            .await
            .map_err(|err| RepoError::from(err, RepoType::Products)).inspect_err(|err|{
                error!("{}",err);
            })?;
        sqlx::query("DELETE FROM products WHERE id=$1")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_or_else(
                |err| Err(RepoError::from(err, RepoType::Products)),
                |result| {
                    if result.rows_affected() != 1 {
                        return Err(RepoError::IdNotExist);
                    }
                    Ok(())
                },
            ).inspect_err(|err|{
                error!("{}",err);
            })?;

        tx.commit()
            .await
            .map_err(|err| RepoError::from(err, RepoType::Products)).inspect_err(|err|{
                error!("{}",err);
            })?;
        Ok(())
    }
}

#[async_trait]
impl ToChangeNotifier for PostgresRepo {
    type Output = Notifier;
    type Err = sqlx::Error;
    async fn to_change_notifier(
        &self,
        cancellation_token: CancellationToken,
        sender: UnboundedSender<TableChanges>,
    ) -> Result<Self::Output, Self::Err> {
        let conn_str = self.0.connect_options().to_url_lossy().to_string();
        let mut listener = PgListener::connect(&conn_str).await?;
        listener.listen("users").await?;
        listener.listen("products").await?;
        listener.ignore_pool_close_event(true);
        Ok(Notifier::new(listener, cancellation_token, sender))
    }
}

#[async_trait]
impl CloseRepository for PostgresRepo {
    async fn close(&self) {
        self.0.close().await;
    }
}
