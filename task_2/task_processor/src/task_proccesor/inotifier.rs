use std::{ffi::OsString, io};

use inotify::{EventMask, Inotify};
use tokio::sync::Mutex;

use crate::errors::InotifierError;

pub struct Inotifier {
    inotifier: Mutex<Option<Inotify>>,
    buffer: Mutex<[u8; 4096]>,
}

impl Inotifier {
    pub fn new(inotify: Inotify) -> Self {
        let buffer = Mutex::new([0; 4096]);
        Self {
            inotifier: Mutex::new(Some(inotify)),
            buffer,
        }
    }

    pub async fn read_events(&self) -> Result<OsString, InotifierError> {
        let mut guard = self.inotifier.lock().await;
        if let Some(inotifier) = &mut *guard {
            let mut buffer = self.buffer.lock().await;
            let buffer = buffer.as_mut_slice();
            let mut events = inotifier.read_events(buffer)?;

            while let Some(event) = events.next() {
                if event.mask.contains(EventMask::MODIFY) {
                    //was created a new file with task
                    if !event.mask.contains(EventMask::ISDIR) {
                        let name = event.name.unwrap().to_owned();
                        return Ok(name);
                    }
                }
            }
            return Err(InotifierError::NotFoundEvent);
        }
        Err(InotifierError::NotFoundSelf)
    }
    pub async fn close(&self) -> io::Result<()> {
        let mut guard = self.inotifier.lock().await;
        if let Some(x) = guard.take() {
            return x.close();
        };
        Ok(())
    }
}
