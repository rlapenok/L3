use std::{error::Error, time::Duration};

use confique::Config;
use deadpool_redis::{
    Config as RedisPoolConfig, ConnectionAddr, ConnectionInfo, Pool as RedisPool,
    RedisConnectionInfo, Runtime::Tokio1,
};
use log::error;

#[derive(Config)]
pub(crate) struct RedisConfig {
    host: String,
    port: u16,
    password: String,
    db: i64,
    max_connections: usize,
    wait: u64,
    create: u64,
}

impl RedisConfig {
    pub async fn create_pool(&self) -> Result<RedisPool, Box<dyn Error>> {
        let connection_address = ConnectionAddr::Tcp(self.host.clone(), self.port);
        RedisConnectionInfo::default();
        let redis_connection_info = RedisConnectionInfo {
            db: self.db,
            password: Some(self.password.clone()),
            ..Default::default()
        };
        let connection_info = ConnectionInfo {
            addr: connection_address,
            redis: redis_connection_info,
        };
        let config = RedisPoolConfig::from_connection_info(connection_info);
        let pool = config
            .builder()?
            .max_size(self.max_connections)
            .wait_timeout(Some(Duration::from_secs(self.wait)))
            .create_timeout(Some(Duration::from_secs(self.create)))
            .runtime(Tokio1)
            .recycle_timeout(None)
            .build()
            .inspect_err(|err| error!("Error while create connection pool Redis: {}", err))?;
        pool.get()
            .await
            .inspect_err(|err| error!("Error while ping Redis: {}", err))?;
        Ok(pool)
    }
}
