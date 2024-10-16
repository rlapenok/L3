use std::{
    error::Error,
    ffi::OsString,
    future::Future,
    path::PathBuf,
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crossbeam::channel::{unbounded, Sender};
use inotify::{Inotify, WatchMask};
use log::{error, info};
use tokio::{
    fs::{self, File},
    io::AsyncReadExt,
    select,
    signal::ctrl_c,
    spawn,
    sync::Mutex,
    task::JoinSet,
};
use tokio_util::sync::CancellationToken;

use crate::{
    errors::{CommonErrors, InotifierError},
    models::ProcessedTask,
};

use super::{
    inotifier::Inotifier,
    job_runner::JobRunner,
    utils::{create_job, write_offset},
    worker::Worker,
};

type WorkerPool = Arc<Mutex<JoinSet<<Worker as Future>::Output>>>;
pub type Job = Pin<Box<dyn Future<Output = Result<ProcessedTask, CommonErrors>> + Send>>;

#[derive(Clone)]
pub struct TaskProccesor {
    inotify: Arc<Inotifier>,
    worker_pool: WorkerPool,
    sender: Arc<Sender<Job>>,
    tasks_directory: Arc<PathBuf>,
    offset_file: Arc<Mutex<File>>,
    job_runner: Arc<dyn JobRunner + Send + Sync>,
}

impl TaskProccesor {
    pub fn new(
        task_dir: PathBuf,
        log_dir: PathBuf,
        num_workers: usize,
        offset_file: File,
        job_runner: Arc<dyn JobRunner + Send + Sync>,
    ) -> Result<Self, Box<dyn Error>> {
        //create channel for send work to workers
        let (sender, recv) = unbounded();
        //create worker_pool
        let log_dir = Arc::new(log_dir);
        let worker_pool = (1..=num_workers).fold(JoinSet::new(), |mut set, id| {
            let worker = Worker::new(id, recv.clone(), log_dir.clone());
            set.spawn(worker);
            set
        });
        //create instans inotify
        let inotify = Inotify::init()
            .inspect_err(|err| error!("Error while creating Inotify instance: {}", err))?;
        inotify
            .watches()
            .add(&task_dir, WatchMask::MODIFY)
            .inspect_err(|err| error!("Error while add WatchMask for Inotify instance: {}", err))?;
        let inotifier = Arc::new(Inotifier::new(inotify));
        info!("TaskProccesod created");
        Ok(Self {
            inotify: inotifier,
            worker_pool: Arc::new(Mutex::new(worker_pool)),
            sender: Arc::new(sender),
            tasks_directory: Arc::new(task_dir),
            offset_file: Arc::new(Mutex::new(offset_file)),
            job_runner,
        })
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
        let mut files = fs::read_dir(&*self.tasks_directory)
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
            if let Some(time_from) = time_from {
                if file_time > time_from {
                    buff.push(file_name);
                }
            } else {
                buff.push(file_name);
            }
        }
        Ok(buff)
    }

    async fn read_events(&self) -> Result<OsString, InotifierError> {
        self.inotify.read_events().await
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
        //task for handle unprocessed tasks while service was stopped
        let unprocessed_tasks_thread = spawn(async move {
            info!("Start thread for unprocessed tasks");

            if let Ok(time_from) = for_unprocessed_tasks_thread.get_offset_from_file().await {
                if let Ok(file_names) = for_unprocessed_tasks_thread
                    .get_unprocessed_tasks(time_from)
                    .await
                {
                    file_names.into_iter().for_each(|file_name| {
                        let path_to_file =
                            for_unprocessed_tasks_thread.tasks_directory.join(file_name);
                        let job = Box::pin(create_job(
                            for_unprocessed_tasks_thread.job_runner.clone(),
                            path_to_file,
                        ));
                        if let Err(err) = for_unprocessed_tasks_thread.sender.send(job) {
                            error!("Error while send job to worker:{}", err)
                        };
                    });
                }
            }
        });
        info!("Start TaskProccesor");
        loop {
            select! {
                result= self.read_events()=>{
                    match result {
                        Ok(data)=>{
                            let path_to_file=self.tasks_directory.join(data);
                            let job=Box::pin(create_job(self.job_runner.clone(), path_to_file));
                            if let Err(err)=self.sender.send(job){
                                error!("Error while send job to worker:{}",err)
                            };
                        }
                        Err(err)=>{
                            if let InotifierError::NotFoundSelf=err{
                                select_token.cancel();
                            }
                        }
                    }
                }
                _ = select_token.cancelled()=>{
                    info!("Start shutdown TaskProccesor");
                    break
                }
            }
        }
        unprocessed_tasks_thread
            .await
            .inspect_err(|err| error!("Error while join unproccesed tasks thread:{}", err))?;
        shutdown_thread
            .await
            .inspect_err(|err| error!("Error while join shutdown thread:{}", err))?;
        self.shutdown().await?;
        Ok(())
    }

    async fn shutdown(self) -> Result<(), Box<dyn Error>> {
        let file = self.offset_file.clone();
        let dir = self.tasks_directory.clone();
        self.inotify
            .close()
            .await
            .inspect_err(|err| error!("Eror while close Inotify instance: {}", err))?;
        drop(self.job_runner);
        //drop sender
        drop(self.sender);
        let mut worker_pool = self.worker_pool.lock().await;
        //wait when workers done
        while let Some(worker_result) = worker_pool.join_next().await {
            if let Err(err) = worker_result {
                error!("Error while join worker: {}", err)
            }
        }
        write_offset(dir, file)
            .await
            .inspect_err(|err| error!("Error while write offset to offset file: {}", err))?;
        info!("Shutdown TaskProccesor completed");
        Ok(())
    }
}
