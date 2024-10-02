use confique::Config;

#[derive(Config, Debug)]
pub(crate) struct ServerConfig {
    host: String,
    port: u16,
    secret: String,
    exp_time_min: i64,
}
impl ServerConfig {
    pub(crate) fn get_listner_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    pub(crate) fn get_info_to_token_manager(self) -> (String, i64) {
        (self.secret, self.exp_time_min)
    }
}
