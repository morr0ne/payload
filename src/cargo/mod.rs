use std::process::{Command, Output};

mod error;
mod metadata;
mod unit_graph;
mod version;

pub use error::{ParsingError, Result};
pub use metadata::Metadata;
pub use unit_graph::UnitGraph;
pub use version::Version;

pub struct Cargo(Command);

impl Cargo {
    pub fn new() -> Self {
        // TODO: Use cargo location since it may not be in the user path.
        Self(Command::new("cargo"))
    }

    fn exec(&mut self) -> Result<Vec<u8>> {
        let Output {
            status,
            stdout,
            stderr,
        } = self.0.output()?;

        if status.success() {
            Ok(stdout)
        } else {
            Err(ParsingError::Exec { stderr })
        }
    }

    pub fn arg(&mut self, arg: &str) -> &mut Self {
        self.0.arg(arg);
        self
    }

    pub fn version(&mut self) -> Result<Version> {
        std::str::from_utf8(&self.arg("-Vv").exec()?)?.parse()
    }

    /// Just for testing, don't use.
    #[doc(hidden)]
    pub fn _build(&mut self) -> Result<UnitGraph> {
        let stdout = self
            .arg("+nightly")
            .arg("build")
            .arg("-Zunstable-options")
            .arg("--unit-graph")
            .exec()?;

        Ok(serde_json::from_slice(&stdout)?)
    }

    pub fn metadata(&mut self) -> Result<Metadata> {
        let stdout = self
            .arg("metadata")
            .arg("--format-version")
            .arg("1")
            .exec()?;

        Ok(serde_json::from_slice(&stdout)?)
    }
}

impl Default for Cargo {
    fn default() -> Self {
        Self::new()
    }
}
