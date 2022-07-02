use std::{io::Error as IoError, str::Utf8Error};
use thiserror::Error;

pub type Result<T, E = ParsingError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("Missing \"{0}\" key when parsing Version")]
    Version(&'static str),
    #[error("Error when executing command")]
    Exec { stderr: Vec<u8> },
    #[error("")]
    Io(#[from] IoError),
    #[error("")]
    Utf8(#[from] Utf8Error),
}
