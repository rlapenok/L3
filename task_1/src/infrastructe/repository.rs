use crate::{
    domain::{
        models::{DeletePost, Post,  User},
        social_network_repository::SocialNetworkRepository,
    },
    errors::ServerErrors,
};
use axum::async_trait;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[derive(Clone)]
pub struct Repository {
    pool: ConnectionPool,
}
impl Repository {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SocialNetworkRepository for Repository {
    async fn register(&self, data: User) -> Result<(), ServerErrors> {
        let mut conn = self
            .pool
            .get()
            .await
            .inspect_err(|err| eprintln!("Error while get connection from pool: {}", err))?;
        let tx_builder = conn.build_transaction();
        let tx = tx_builder
            .start()
            .await
            .inspect_err(|err| eprintln!("Error while start transaction: {}", err))?;
        match tx
            .execute(
                "INSERT INTO users (user_uid, login, hashed_password) VALUES ($1, $2, $3)",
                &[&data.user_uid, &data.login, &data.hashed_password],
            )
            .await
        {
            Ok(_) => {
                tx.commit()
                    .await
                    .inspect_err(|err| eprintln!("Error while commit transaction: {}", err))
                    .unwrap();
                Ok(())
            }
            Err(err) => {
                tx.rollback()
                    .await
                    .inspect_err(|err| eprintln!("Error while rollback transaction :{}", err))?;
                Err(ServerErrors::from(err))
            }
        }
    }
    async fn login(&self, data: User) -> Result<(), ServerErrors> {
        let conn = self
            .pool
            .get()
            .await
            .inspect_err(|err| eprintln!("Error while get connection from pool: {}", err))?;
        conn
            .query_opt(
                "SELECT user_uid FROM users WHERE user_uid=$1 AND login=$2 AND hashed_password=$3",
                &[&data.user_uid, &data.login, &data.hashed_password],
            )
            .await?.ok_or(ServerErrors::NotFindUser)?;
        Ok(())
        }
    async fn create_post(&self, data: Post) -> Result<(), ServerErrors> {
        let mut conn = self
            .pool
            .get()
            .await
            .inspect_err(|err| eprintln!("Error while get connection from pool: {}", err))?;
        let tx_builder = conn.build_transaction();
        let tx = tx_builder
            .start()
            .await
            .inspect_err(|err| eprintln!("Error while start transaction: {}", err))?;
        match tx
            .execute(
                "INSERT INTO posts (user_uid, post_uid, msg,likes) VALUES ($1, $2, $3, $4)",
                &[&data.user_uid, &data.post_uid, &data.msg, &data.likes],
            )
            .await
        {
            Ok(_) => {
                tx.commit()
                    .await
                    .inspect_err(|err| eprintln!("Error while commit transaction: {}", err))?;
                Ok(())
            }
            Err(err) => {
                tx.rollback()
                    .await
                    .inspect_err(|err| eprintln!("Error while rollback transaction :{}", err))?;
                Err(ServerErrors::from(err))
            }
        }
    }
    async fn get_post(&self, post_uid: Uuid) -> Result<Post, ServerErrors> {
        let conn = self
            .pool
            .get()
            .await
            .inspect_err(|err| eprintln!("Error while get connection from pool: {}", err))?;
        let row=conn
            .query_opt("SELECT * FROM posts WHERE post_uid=$1", &[&post_uid])
            .await?.ok_or(ServerErrors::NotFindPost)?;
        let post=Post::try_from(row)?;
        Ok(post)

    }

    async fn delete_post(&self,post: DeletePost)->Result<(),ServerErrors>{
        println!("{:?}",post);
        let mut conn = self
        .pool
        .get()
        .await
        .inspect_err(|err| eprintln!("Error while get connection from pool: {}", err))?;
    let tx_builder = conn.build_transaction().read_only(false);
    let tx = tx_builder
        .start()
        .await
        .inspect_err(|err| eprintln!("Error while start transaction: {}", err))?;
    match tx
        .execute("DELETE FROM posts WHERE user_uid =$1 and post_uid =$2", &[&post.user_uid,&post.post_uid])
        .await
    {
        Ok(row) => {
                if row==1{
                    tx.commit()
            .await
            .inspect_err(|err| eprintln!("Error while commit transaction: {}", err))?;
                 return Ok(())
                }else {
                    return  Err(ServerErrors::NotFindPost);
                }
        }
        Err(err) => {
            tx.rollback()
                .await
                .inspect_err(|err| eprintln!("Error while rollback transaction :{}", err))?;
            Err(ServerErrors::from(err))
        }
    }
    }
    async fn like_post(&self,post_uid: Uuid)->Result<(),ServerErrors>{
        let mut conn = self
        .pool
        .get()
        .await
        .inspect_err(|err| eprintln!("Error while get connection from pool: {}", err))?;
    let tx_builder = conn.build_transaction();
    let tx = tx_builder
        .start()
        .await
        .inspect_err(|err| eprintln!("Error while start transaction: {}", err))?;
    match tx
        .execute("UPDATE posts SET likes=likes+1 WHERE post_uid=$1", &[&post_uid])
        .await
    {
        Ok(row) => {
            if row==1{
                tx.commit()
        .await
        .inspect_err(|err| eprintln!("Error while commit transaction: {}", err))?;
             return Ok(())
            }else {
                return  Err(ServerErrors::NotFindPost);
            }
        }
        Err(err) => {
            tx.rollback()
                .await
                .inspect_err(|err| eprintln!("Error while rollback transaction :{}", err))?;
            Err(ServerErrors::from(err))
        }
    }
    }
}