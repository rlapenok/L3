use std::{future::Future, path::PathBuf, pin::Pin, sync::Arc};

use crossbeam::channel::Receiver;

use log::{error, info};
use tokio::{
    fs::OpenOptions,
    io::AsyncWriteExt,
    spawn,
    task::{JoinError, JoinHandle},
};

use crate::{errors::CommonErrors, models::ProcessedTask};

use super::proccesor::Job;

pub(crate) struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, recv: Receiver<Job>, dir: Arc<PathBuf>) -> Self {
        let handle = spawn(async move {
            while let Ok(job) = recv.recv() {
                info!("Worker № {}---recv new task", id);
                match job.await {
                    Ok(processed_task) => {
                        let task_uuid = processed_task.uuid;
                        if let Err(err) = save_processed_task(dir.clone(), processed_task).await {
                            error!(
                                "Worker № {} task_uuid :{}---Error while save processed task: {}",
                                id, task_uuid, err
                            )
                        }
                    }
                    Err(err) => {
                        error!("Worker № {}---Error while run job: {}", id, err);
                    }
                }
            }
        });
        Self { handle }
    }
}

impl Future for Worker {
    type Output = Result<(), JoinError>;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Pin::new(&mut self.get_mut().handle).poll(cx)
    }
}
async fn save_processed_task(
    dir: Arc<PathBuf>,
    processed_task: ProcessedTask,
) -> Result<(), CommonErrors> {
    let file_name = format!("{}.json", processed_task.uuid);
    let path = dir.join(file_name);
    let info = serde_json::to_string_pretty(&processed_task)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await?;
    file.write_all(info.as_bytes()).await?;
    Ok(())
}
