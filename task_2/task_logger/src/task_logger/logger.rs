use std::{
    error::Error,
    ffi::OsString,
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use inotify::{Inotify, WatchMask};
use log::{error, info};
use tokio::{
    fs::{self, File},
    io::AsyncReadExt,
    select,
    signal::ctrl_c,
    spawn,
    sync::Mutex,
};
use tokio_util::sync::CancellationToken;

use crate::{
    errors::{CommonErrors, InotifierError},
    task_logger::utils::write_offset,
};

use super::inotifier::Inotifier;

#[derive(Clone)]
pub struct TaskLogger {
    inotify: Arc<Inotifier>,
    offset_file: Arc<Mutex<File>>,
    log_directory: Arc<PathBuf>,
}

impl TaskLogger {
    pub fn new(log_dir: PathBuf, offset_file: File) -> Result<Self, Box<dyn Error>> {
        let inotifier = Inotify::init()
            .inspect_err(|err| error!("Error while creating Inotify instance: {}", err))?;
        inotifier
            .watches()
            .add(&log_dir, WatchMask::MODIFY)
            .inspect_err(|err| error!("Error while add WatchMask for Inotify instance: {}", err))?;
        let log_directory = Arc::new(log_dir);
        let inotifier = Arc::new(Inotifier::new(inotifier));
        Ok(Self {
            inotify: inotifier,
            offset_file: Arc::new(Mutex::new(offset_file)),
            log_directory,
        })
    }
    async fn read_events(&self) -> Result<OsString, InotifierError> {
        self.inotify.read_events().await
    }

    //get offset from file
    async fn get_offset_from_file(&self) -> Result<Option<SystemTime>, CommonErrors> {
        let mut guard = self.offset_file.lock().await;
        let mut buff = String::new();
        guard
            .read_to_string(&mut buff)
            .await
            .inspect_err(|err| error!("Error while read ofset file: {}", err))?;
        guard
            .set_len(0)
            .await
            .inspect_err(|err| error!("Error cleaning offet file: {}", err))?;
        if !buff.is_empty() {
            let time = buff
                .parse::<u64>()
                .inspect_err(|err| error!("Error while parse offset from offset file: {}", err))?;
            return Ok(Some(UNIX_EPOCH + Duration::from_nanos(time)));
        }
        Ok(None)
    }

    //get unprocessed tasks from tasks directory
    async fn get_unprocessed_tasks(
        &self,
        time_from: Option<SystemTime>,
    ) -> Result<Vec<String>, CommonErrors> {
        let mut files = fs::read_dir(&*self.log_directory)
            .await
            .inspect_err(|err| error!("Error while read tasks directory: {}", err))?;
        let mut buff = Vec::new();
        while let Some(file) = files
            .next_entry()
            .await
            .inspect_err(|err| error!("Error while interation on tasks directory: {}", err))?
        {
            let file_time = file
                .metadata()
                .await
                .inspect_err(|err| error!("Error while get metadata fom file with task :{}", err))?
                .modified()
                .inspect_err(|err| {
                    error!("Error while get modified time from file with task: {}", err)
                })?;
            let file_name = file.file_name().to_string_lossy().to_string();
            let task_uuid = file_name.split(".").collect::<Vec<&str>>()[0].to_owned();
            if let Some(time_from) = time_from {
                if file_time > time_from {
                    buff.push(task_uuid);
                }
            } else {
                buff.push(task_uuid);
            }
        }
        Ok(buff)
    }

    async fn shutdown(&self) -> Result<(), Box<dyn Error>> {
        write_offset(self.log_directory.clone(), self.offset_file.clone())
            .await
            .inspect_err(|err| error!("Error while write offset to offset file: {}", err))?;
        info!("Shutdown TaskPeoccesor completed");
        Ok(())
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        //create token for cancellation
        let ctrlc_token = Arc::new(CancellationToken::new());
        // token for select
        let select_token = ctrlc_token.clone();
        //backgorund task for handle ctrl_c signal
        let shutdown_thread = spawn(async move {
            ctrl_c().await.expect("failed to install Ctrl+C handler");
            ctrlc_token.cancel();
        });
        let for_unprocessed_tasks_thread = self.clone();

        let unprocessed_tasks_thread = spawn(async move {
            info!("Start thread for unprocessed tasks");

            if let Ok(time_from) = for_unprocessed_tasks_thread.get_offset_from_file().await {
                if let Ok(tasks_uuid) = for_unprocessed_tasks_thread
                    .get_unprocessed_tasks(time_from)
                    .await
                {
                    tasks_uuid
                        .into_iter()
                        .for_each(|task_uuid| info!("Task № {} processed", task_uuid));
                }
            }
            info!("Stop thread for unprocessed tasks");
        });
        info!("Start TaskLogger");
        loop {
            select! {
                result= self.read_events()=>{
                    match result {
                        Ok(data)=>{
                           if let Some(file_name)=data.to_str().to_owned(){
                                let task_uuid=file_name.split(".").collect::<Vec<&str>>()[0];
                                info!("Task № {} processed",task_uuid)
                           }
                        }
                        Err(err)=>{
                            if let InotifierError::NotFoundSelf=err{
                                select_token.cancel();
                            }
                        }
                    }
                }
                _ = select_token.cancelled()=>{
                    info!("Start shutdown TaskLogger");
                    break
                }
            }
        }

        unprocessed_tasks_thread.await?;
        shutdown_thread.await?;
        self.shutdown().await?;
        Ok(())
    }
}
