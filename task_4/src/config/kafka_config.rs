use std::error::Error;

use confique::Config;
use rdkafka::{producer::FutureProducer, ClientConfig};

#[derive(Config)]
pub(crate) struct KafkaConfig {
    topic_name: String,
    addr: String,
}

impl KafkaConfig {
    pub(crate) fn to_producer(&self) -> Result<FutureProducer, Box<dyn Error>> {
        let prod = ClientConfig::new()
            .set("bootstrap.servers", &self.addr)
            .set("message.timeout.ms", "5000")
            .set("allow.auto.create.topics", "true")
            .create::<FutureProducer>()?;
        Ok(prod)
    }
    pub(crate) fn get_topic_name(&self) -> &str {
        &self.topic_name
    }
}
