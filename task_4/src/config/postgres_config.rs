use std::time::Duration;

use confique::Config;
use sqlx::{pool::PoolOptions, postgres::PgConnectOptions, Postgres};

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
    pub fn to_connect_opt(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .database(&self.db)
            .username(&self.login)
            .password(&self.password)
    }
    pub fn to_pool_opt(&self) -> PoolOptions<Postgres> {
        PoolOptions::new()
            .max_lifetime(Some(Duration::from_secs(self.max_lifetime_sec)))
            .idle_timeout(Some(Duration::from_secs(self.idle_timeout_sec)))
            .max_connections(self.max_connections)
            .test_before_acquire(true)
            .acquire_timeout(Duration::from_secs(self.acquire_time_sec))
    }
}
