use std::{error::Error, time::Duration};

use confique::Config;
use sqlx::{pool::PoolOptions, postgres::PgConnectOptions, PgPool, Postgres};

#[derive(Config)]
pub(crate) struct PostgresConfig {
    host: String,
    port: u16,
    login: String,
    password: String,
    db: String,
    max_connections: u32,
    idle_timeout_sec: u64,
    max_lifetime_sec: u64,
    acquire_time_sec: u64,
}

impl PostgresConfig {
    fn to_connect_opt(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .database(&self.db)
            .username(&self.login)
            .password(&self.password)
    }
    fn to_pool_opt(&self) -> PoolOptions<Postgres> {
        PoolOptions::new()
            .max_lifetime(Some(Duration::from_secs(self.max_lifetime_sec)))
            .idle_timeout(Some(Duration::from_secs(self.idle_timeout_sec)))
            .max_connections(self.max_connections)
            .test_before_acquire(true)
            .acquire_timeout(Duration::from_secs(self.acquire_time_sec))
    }
    pub async fn to_pool(&self) -> Result<PgPool, Box<dyn Error>> {
        let conn_opt = self.to_connect_opt();
        let pool_opt = self.to_pool_opt();
        let pool = pool_opt.connect_with(conn_opt).await?;
        Ok(pool)
    }
}
