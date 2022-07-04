use std::{io::Error as IoError, str::Utf8Error};
use thiserror::Error;

pub type Result<T, E = ParsingError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("Missing \"{0}\" key when parsing Version")]
    Version(&'static str),
    #[error(
        "Error when executing command. The following is the stderr output:\n{0}",
        String::from_utf8_lossy(stderr)
    )]
    Exec { stderr: Vec<u8> },
    #[error("")]
    Io(#[from] IoError),
    #[error("")]
    Utf8(#[from] Utf8Error),
    #[error("")]
    Semver(#[from] semver::Error),
    #[error("")]
    Triple(#[from] target_lexicon::ParseError),
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
}
