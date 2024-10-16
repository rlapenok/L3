use std::{io, num::ParseIntError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InotifierError {
    #[error("Error while read event:{0}")]
    ReadError(#[from] io::Error),
    #[error("Event not found")]
    NotFoundEvent,
    #[error("Inofier is None")]
    NotFoundSelf,
}

#[derive(Debug, Error)]
pub enum CommonErrors {
    #[error("Error while read files from directory:{0}")]
    ReadError(#[from] io::Error),
    #[error("Not found last file in directory")]
    ParseOffsetData(#[from] ParseIntError),
    #[error("Error while deserialize task:{0}")]
    DeserializeError(#[from] serde_json::Error),
}
