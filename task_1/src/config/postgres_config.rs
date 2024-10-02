use confique::Config;
use tokio_postgres::{Config as TokioPostgresConfig, NoTls};

#[derive(Config, Debug)]
pub struct PostgresConfig {
    host: String,
    port: u16,
    login: String,
    password: String,
    db: String,
}

impl PostgresConfig {
    pub fn get_connection_config(&self) -> (TokioPostgresConfig, NoTls) {
        let mut config = TokioPostgresConfig::new();
        config.host(&self.host);
        config.port(self.port);
        config.user(&self.login);
        config.password(&self.password);
        config.dbname(&self.db);
        (config, NoTls)
    }
}
