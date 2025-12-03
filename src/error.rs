use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("ValueError: {0}")]
    ValueError(String),
}
