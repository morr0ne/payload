use std::{
    env,
    ffi::OsStr,
    path::PathBuf,
    process::{Command, Output},
};

use which::which;

pub mod error;
pub mod metadata;
pub mod unit_graph;
pub mod version;

pub use error::{ParsingError, Result};
pub use metadata::Metadata;
pub use unit_graph::UnitGraph;
pub use version::Version;

pub struct Cargo {
    path: PathBuf,
}

impl Cargo {
    pub fn new() -> Self {
        // Check the "CARGO" enviroment variable, if not found try running which on "cargo", if that also doesn't work just use "cargo".
        let path = env::var("CARGO")
            .map(PathBuf::from)
            .unwrap_or_else(|_| which("cargo").unwrap_or_else(|_| PathBuf::from("cargo")));
        dbg!(&path);
        Self { path }
    }

    pub fn command(&self) -> Command {
        Command::new(&self.path)
    }

    fn exec<I, S>(&self, args: I) -> Result<Vec<u8>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let Output {
            status,
            stdout,
            stderr,
        } = self.command().args(args).output()?;

        if status.success() {
            Ok(stdout)
        } else {
            Err(ParsingError::Exec { stderr })
        }
    }

    pub fn version(&mut self) -> Result<Version> {
        std::str::from_utf8(&self.exec(["-Vv"])?)?.parse()
    }

    /// Just for testing, don't use.
    #[doc(hidden)]
    pub fn _build(&mut self) -> Result<UnitGraph> {
        let stdout = self.exec(&["+nightly", "build", "-Zunstable-options", "--unit-graph"])?;

        Ok(serde_json::from_slice(&stdout)?)
    }

    pub fn metadata(&mut self) -> Result<Metadata> {
        let stdout = self.exec(&["metadata", "--format-version", "1"])?;

        Ok(serde_json::from_slice(&stdout)?)
    }
}

impl Default for Cargo {
    fn default() -> Self {
        Self::new()
    }
}
