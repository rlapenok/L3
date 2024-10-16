use std:: sync::Arc;

use crate::{domain::{models::Task, task_saver::TaskSaver}, errors::ServerError};




#[derive(Clone)]
pub struct ServerState
{
    saver:Arc<dyn TaskSaver>
}

impl  ServerState
{
    pub fn new(saver:Arc<dyn TaskSaver>)->Self{
        Self { saver }
    }


    pub async fn save_task(&self,task:Task)->Result<(),ServerError>{
        self.saver.save_task(task).await
    }
}







/*#[derive(Clone)]
pub struct ServerState {
    file: Arc<Mutex<File>>,
}

impl ServerState {
    pub fn new(file:File) -> Self{
        Self { file: Arc::new(Mutex::new(file)) }
    }
    pub async fn save_task(&self,task:NewTask)->Result<(),ServerError>{
        let mut guard=self.file.lock().await;
        let mut task=serde_json::to_string_pretty(&task)?;
            guard.lock_exclusive()?;
        task.push_str("\r\n#\r\n");
          guard.write_all(task.as_bytes()).await?;
        guard.unlock()?;  
        Ok(())

        
    }
}*/
