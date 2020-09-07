use std::io;
use thiserror;
#[derive(thiserror::Error, Debug)]
pub enum HashitError {
    #[error("for {file} - {source}")]
    NotFound {
        source: io::Error,
        file: std::path::PathBuf,
    },
    #[error(transparent)]
    //#[error("IO Error: {0}")]
    IoError(#[from] io::Error),

    #[error("Directory does not exist: '{0}'")]
    MissingDir(String),

    #[error("Key does not exist: '{0}'")]
    MissingKey(String),

    #[error("{0} Not Implemented")]
    NotImplemented(String),
}

pub type Result<R> = std::result::Result<R, HashitError>;
