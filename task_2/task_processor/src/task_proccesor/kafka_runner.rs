use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::{path::PathBuf, sync::Arc, time::Duration};

use crate::{
    errors::CommonErrors, models::ProcessedTask, task_proccesor::utils::get_task_from_file,
};

use super::job_runner::JobRunner;

#[derive(Clone)]
pub struct KafkaRunner {
    prod: FutureProducer,
    topic: Arc<String>,
}
impl KafkaRunner {
    pub fn new(prod: FutureProducer, topic: String) -> Self {
        Self {
            prod,
            topic: Arc::new(topic),
        }
    }
}
#[async_trait]
impl JobRunner for KafkaRunner {
    async fn get_gob(&self, path_to_task: PathBuf) -> Result<ProcessedTask, CommonErrors> {
        let prod = self.prod.clone();
        let created_task = get_task_from_file(path_to_task).await?;
        let payload = serde_json::to_string_pretty(&created_task)?;
        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key("task_2");
        let proccesed_task = prod
            .send(record, Duration::from_millis(500))
            .await
            .map_or_else(
                |err| {
                    ProcessedTask::new(
                        created_task.uuid,
                        created_task.created_at,
                        Err(err.0.to_string()),
                    )
                },
                |_| ProcessedTask::new(created_task.uuid, created_task.created_at, Ok(())),
            );
        Ok(proccesed_task)
    }
}
