use std::{
    env::{self},
    error::Error,
    path::PathBuf,
    sync::Arc,
};

use clap::{Parser, Subcommand};
use log::error;
use rdkafka::{producer::FutureProducer, ClientConfig};
use tokio::fs::{create_dir_all, OpenOptions};

use crate::task_proccesor::{
    console_runner::ConsoleRunner, job_runner::JobRunner, kafka_runner::KafkaRunner,
    proccesor::TaskProccesor,
};

#[derive(Subcommand)]
pub enum TaskType {
    Kafka,
    Console,
}

#[derive(Parser)]
pub struct TaskProccesorCli {
    #[arg(short = 'p', long)]
    path_to_tasks_dir: PathBuf,
    #[arg(short = 'l', long)]
    path_to_log_dir: PathBuf,
    #[arg(short = 'n', long)]
    num_workers: usize,
    #[command(subcommand)]
    task_type: TaskType,
}

impl TaskProccesorCli {
    pub async fn to_task_processor(self) -> Result<TaskProccesor, Box<dyn Error>> {
        //get file_name for save offset
        let path_to_offset_file = env::var("OFFSET_FILE")
            .inspect_err(|err| error!("Error while get OFFSET_FILE from .env: {}", err))?;
        //create offset_file
        let offset_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path_to_offset_file.trim())
            .await
            .inspect_err(|err| error!("Error create file from env OFFSET_FILE: {}", err))?;
        //create directory for logs
        create_dir_all(&self.path_to_log_dir)
            .await
            .inspect_err(|err| error!("Error while cread directory for log directory: {}", err))?;
        //create runner
        let job_runner = match self.task_type {
            TaskType::Console => Arc::new(ConsoleRunner) as Arc<dyn JobRunner + Send + Sync>,
            TaskType::Kafka => {
                // get addres to kafka
                let kafka_addr = env::var("KAFKA_ADDR")
                    .inspect_err(|err| error!("Error while get KAFKA_ADDR from .env: {}", err))?;
                //get topic name for kafka
                let topic = env::var("KAFKA_TOPIC")
                    .inspect_err(|err| error!("Error while get KAFKA_TOPIC from .env: {}", err))?;
                //create producer for kafka
                let producer = ClientConfig::new()
                    .set("bootstrap.servers", kafka_addr)
                    .set("message.timeout.ms", "5000")
                    .set("allow.auto.create.topics", "true")
                    .create::<FutureProducer>()
                    .inspect_err(|err| error!("Error while create kafka producer: {}", err))?;
                Arc::new(KafkaRunner::new(producer, topic)) as Arc<dyn JobRunner + Send + Sync>
            }
        };
        //create task proccesor
        TaskProccesor::new(
            self.path_to_tasks_dir,
            self.path_to_log_dir,
            self.num_workers,
            offset_file,
            job_runner,
        )
    }
}
