use confique::Config;
use redis::{ConnectionAddr, ConnectionInfo, RedisConnectionInfo};

#[derive(Config)]
pub(crate) struct RedisConfig {
    host: String,
    port: u16,
    password: String,
    db: i64,
}

impl RedisConfig {
    pub fn to_connect_info(&self) -> ConnectionInfo {
        let connection_address = ConnectionAddr::Tcp(self.host.clone(), self.port);
        let redis_connection_info = RedisConnectionInfo {
            db: self.db,
            password: Some(self.password.clone()),
            ..Default::default()
        };
        ConnectionInfo {
            addr: connection_address,
            redis: redis_connection_info,
        }
    }
}
