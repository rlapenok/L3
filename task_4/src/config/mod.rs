use std::{error::Error, path::PathBuf};

use confique::Config;
use kafka_config::KafkaConfig;
use postgres_config::PostgresConfig;
use sqlx::PgPool;
use tokio::sync::mpsc::unbounded_channel;
use tokio_util::sync::CancellationToken;

use crate::{
    domain::{
        change_notifier::TableChangeNotifier, kafka_sender::KafkaSender, utils::ToChangeNotifier,
    },
    infrastructe::{
        kafka_producer::KafkaProducer, notifier::Notifier, postgres_repo::PostgresRepo,
        server_state::ServerState,
    },
};

mod kafka_config;
mod postgres_config;
mod utils;

#[derive(Config)]
pub struct AppConfig {
    #[config(nested)]
    postgres: PostgresConfig,
    #[config(nested)]
    kafka: KafkaConfig,
}

impl AppConfig {
    pub fn load(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let config = Self::from_file(path)?;
        Ok(config)
    }

    async fn to_postgres_pool(&self) -> Result<PgPool, sqlx::Error> {
        let conn_opt = self.postgres.to_connect_opt();
        let pool_opt = self.postgres.to_pool_opt();
        pool_opt.connect_with(conn_opt).await
    }
    async fn to_postgres_repo(&self) -> Result<PostgresRepo, Box<dyn Error>> {
        let pool = self.to_postgres_pool().await?;
        let repo = PostgresRepo(pool);
        Ok(repo)
    }
    pub async fn to_state(
        &self,
    ) -> Result<ServerState<PostgresRepo, Notifier, KafkaProducer>, Box<dyn Error>> {
        let repo = self.to_postgres_repo().await?;
        //create cancellation_token for notifier and kafka sender
        let cancellation_token = CancellationToken::new();
        let cancellation_token_for_notifier = cancellation_token.child_token();
        let cancellation_token_for_kafka_producer = cancellation_token.child_token();

        //create channel for tables change notifier and kafka producer
        let (sender, receiver) = unbounded_channel();
        //create change notifier
        let notifier = repo
            .to_change_notifier(cancellation_token_for_notifier, sender)
            .await?;
        let notifier_task = notifier.run_notifier();
        //create kafka sender
        let prod = self.kafka.to_producer()?;
        let topic_name = self.kafka.get_topic_name();
        let kafka_producer = KafkaProducer::new(
            topic_name.to_owned(),
            prod,
            receiver,
            cancellation_token_for_kafka_producer,
        );
        let kafka_task = kafka_producer.run_sender();
        let state = ServerState::new(repo, notifier, cancellation_token, kafka_producer);
        state.add_task_to_tracker(notifier_task);
        state.add_task_to_tracker(kafka_task);
        Ok(state)
    }
}
