use std::io;
use thiserror;
#[derive(thiserror::Error, Debug)]
pub enum HashitError {
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),

    #[error("Directory does not exist: '{0}'")]
    MissingDir(String),
}

pub type HashitResult<R> = Result<R, HashitError>;
