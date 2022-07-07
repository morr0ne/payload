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
    frozen: bool,
    locked: bool,
    offline: bool,
}

impl Cargo {
    pub fn new() -> Self {
        // Check the "CARGO" enviroment variable, if not found try running which on "cargo", if that also doesn't work just use "cargo".
        let path = env::var("CARGO")
            .map(PathBuf::from)
            .unwrap_or_else(|_| which("cargo").unwrap_or_else(|_| PathBuf::from("cargo")));

        Self {
            path,
            frozen: false,
            locked: false,
            offline: false,
        }
    }

    /// Require Cargo.lock and cache are up to date.
    pub fn frozen(&mut self, frozen: bool) -> &mut Self {
        self.frozen = frozen;
        self
    }

    /// Require Cargo.lock is up to date.
    pub fn locked(&mut self, locked: bool) -> &mut Self {
        self.locked = locked;
        self
    }

    /// Run without accessing the network.
    pub fn offline(&mut self, offline: bool) -> &mut Self {
        self.offline = offline;
        self
    }

    /// Sets the path to the `cargo` executable.
    ///
    /// The default one is set by first checking the "CARGO" enviroment variable,
    /// if not found  running trying [which](which::which) on "cargo",
    /// if that also doesn't work just uses the string "cargo".
    pub fn path<P: Into<PathBuf>>(&mut self, path: P) -> &mut Self {
        self.path = path.into();
        self
    }

    pub fn command(&self) -> Command {
        let mut command = Command::new(&self.path);

        if self.frozen {
            command.arg("--frozen");
        }

        if self.locked {
            command.arg("--locked");
        }

        if self.offline {
            command.arg("--offline");
        }

        command
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
