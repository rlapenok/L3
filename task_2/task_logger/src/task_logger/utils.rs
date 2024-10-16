use std::{error::Error, sync::Arc};

use tokio::{
    fs::{read_dir, File},
    io::AsyncWriteExt,
    sync::Mutex,
};

use std::{path::PathBuf, time::UNIX_EPOCH};

pub async fn write_offset(dir: Arc<PathBuf>, file: Arc<Mutex<File>>) -> Result<(), Box<dyn Error>> {
    let mut files = read_dir(&*dir).await?;
    let mut offset_time = 0;
    while let Some(file) = files.next_entry().await? {
        if let Ok(meta) = file.metadata().await {
            if let Ok(time) = meta.created() {
                if let Ok(time) = time.duration_since(UNIX_EPOCH) {
                    if offset_time < time.as_nanos() {
                        offset_time = time.as_nanos()
                    }
                }
            }
        }
    }
    let mut guard = file.lock().await;
    guard.write_all(offset_time.to_string().as_bytes()).await?;
    Ok(())
}
