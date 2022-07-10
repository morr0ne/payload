use std::path::PathBuf;
use which::which;

pub struct Rustc {
    path: PathBuf,
}

impl Rustc {
    pub fn new() -> Self {
        let path = which("cargo").unwrap_or_else(|_| PathBuf::from("cargo"));

        Rustc { path }
    }
}

impl Default for Rustc {
    fn default() -> Self {
        Self::new()
    }
}
