use std::{
    io,
    process::{Command, Output},
};

mod error;
mod version;

pub use error::{ParsingError, Result};
pub use version::Version;

pub struct Cargo(Command);

impl Cargo {
    pub fn new() -> Self {
        // TODO: Use cargo location since it may not be in the user path.
        Self(Command::new("cargo"))
    }

    fn exec(&mut self) -> io::Result<Output> {
        self.0.output()
    }

    pub fn arg(&mut self, arg: &str) -> &mut Self {
        self.0.arg(arg);
        self
    }

    pub fn version(&mut self) -> Result<Version> {
        let Output {
            status,
            stdout,
            stderr,
        } = self.arg("-Vv").exec()?;

        if status.success() {
            std::str::from_utf8(&stdout)?.parse()
        } else {
            Err(ParsingError::Exec { stderr })
        }
    }
}

impl Default for Cargo {
    fn default() -> Self {
        Self::new()
    }
}
