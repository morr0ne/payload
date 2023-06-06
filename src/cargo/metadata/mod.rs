use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

mod config;
pub use config::{Features, MetadataConfig};

/// The parsed output of `cargo metadata`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Metadata {
    /// Array of all packages in the workspace.
    /// It also includes all feature-enabled dependencies unless --no-deps is used.
    pub packages: Vec<Package>,
    /// Array of members of the workspace.
    /// Each entry is the Package ID for the package.
    pub workspace_members: Vec<String>, // TODO: can we potentially parse package id? Looking at cargo's source suggests we can
    /// The resolved dependency graph for the entire workspace. The enabled
    /// features are based on the enabled features for the "current" package.
    /// Inactivated optional dependencies are not listed.
    ///
    /// This is [None] if --no-deps is specified.
    ///
    /// By default, this includes all dependencies for all target platforms.
    /// The `--filter-platform` flag may be used to narrow to a specific
    /// target triple.
    pub resolve: Option<Resolve>,
    /// The absolute path to the build directory where Cargo places its output.
    pub target_directory: PathBuf,
    /// The version of the schema for this metadata structure.
    /// This will be changed if incompatible changes are ever made.
    pub version: usize,
    /// The absolute path to the root of the workspace.
    pub workspace_root: PathBuf,
    /// Workspace metadata.
    /// This is [None] if no metadata is specified. */
    #[serde(rename = "metadata")]
    pub workspace_metadata: Option<Value>,
}

/// A single rust package.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Package {
    /// The name of the package.
    pub name: String,
    /// The version of the package.
    pub version: String,
    /// The Package ID, a unique identifier for referring to the package.
    pub id: String, // TODO: Parse as package id
    /// The license value from the manifest.
    pub license: Option<String>, // TODO: Maybe parse using spdx crate? Not sure if that would cause too much issues with failed parsing.
    /// The license-file value from the manifest.
    pub license_file: Option<String>,
    /// The description value from the manifest.
    pub description: Option<String>,
    /// The source ID of the package. This represents where
    /// a package is retrieved from.
    /// This is null for path dependencies and workspace members.
    /// For other dependencies, it is a string with the format:
    /// - "registry+URL" for registry-based dependencies.
    ///   Example: "registry+https://github.com/rust-lang/crates.io-index"
    /// - "git+URL" for git-based dependencies.
    ///   Example: "git+https://github.com/rust-lang/cargo?rev=5e85ba14aaa20f8133863373404cb0af69eeef2c#5e85ba14aaa20f8133863373404cb0af69eeef2c"
    pub source: Option<String>, // TODO: Maybe parse the url?
    /// Array of dependencies declared in the package's manifest.
    pub dependencies: Vec<Dependency>, // TODO: unsure if this is the same as unit graph
    /// Array of Cargo targets.
    pub targets: Vec<Target>,
    /// Set of features defined for the package.
    pub features: HashMap<String, Vec<String>>,
    /// Absolute path to this package's manifest.
    pub manifest_path: PathBuf,
    /// Package metadata.
    #[serde(rename = "metadata")]
    pub package_metadata: Option<Value>,
    /// List of registries to which this package may be published.
    ///
    /// To access the underlying type use [Publishing]
    pub publish: Publishing, // TODO: Maybe better names for the  structs?
    /// Array of authors from the manifest.
    /// Empty array if no authors specified.
    pub authors: Vec<String>,
    /// Array of categories from the manifest.
    pub categories: Vec<String>,
    /// The default binary picked by cargo run.
    pub default_run: Option<String>,
    /// The minimum supported rust version.
    pub rust_version: Option<String>,
    /// Array of keywords from the manifest.
    pub keywords: Vec<String>,
    /// The readme value from the manifest or [None] if not specified.
    pub readme: Option<String>,
    /// The repository value from the manifest or [None] if not specified.
    pub repository: Option<String>,
    /// The homepage value from the manifest or [None] if not specified.
    pub homepage: Option<String>,
    /// The documentation value from the manifest or [None] if not specified.
    pub documentation: Option<String>,
    /// The default edition of the package.
    /// Note that individual targets may have different editions.
    pub edition: Edition,
    /// The name of a native library the package is linking to.
    pub links: Option<String>,
}

/// The publishing restrictions of this package.
///
/// Call `restrictions` to get the actual restrictions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct Publishing(Option<Vec<String>>);

impl Publishing {
    /// Get the underlying publishing restrictions
    pub fn restrictions(&self) -> PublishingRestrictions {
        if let Some(ref registries) = self.0 {
            if registries.is_empty() {
                PublishingRestrictions::Forbidden
            } else {
                PublishingRestrictions::Registries(registries)
            }
        } else {
            PublishingRestrictions::Unrestricted
        }
    }
}

/// The kind of publishing restrictions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublishingRestrictions<'a> {
    Unrestricted,
    Forbidden,
    Registries(&'a Vec<String>),
}

/// A dependency declared in the package's manifest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dependency {
    /// The name of the dependency.
    pub name: String,
    pub source: Option<String>,
    pub req: String,
    pub kind: Option<String>, // TODO: Parse as an enum.
    pub rename: Option<String>,
    pub optional: bool,
    pub uses_default_features: bool,
    pub features: Vec<String>,
    pub target: Option<String>, // TODO: Maybe parse via cargo_platform
    pub path: Option<PathBuf>,
    pub registry: Option<String>, // TODO: Maybe parse using url crate.
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Target {
    /// An array of target kinds
    pub kind: Vec<TargetKind>,
    pub crate_types: Vec<String>, // TODO: Not sure if it should be TargetKind or anotther enum.
    pub name: String,
    pub src_path: PathBuf,
    pub edition: Edition,
    #[serde(rename = "required-features")]
    pub required_features: Option<Vec<String>>,
    pub doc: bool,
    pub doctest: bool,
    /// Whether or not this target should be built and run with `--test`
    pub test: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetKind {
    /// A runnable executable.
    #[serde(rename = "bin")]
    Bin,
    /// A Rust library.
    #[serde(rename = "lib")]
    Lib,
    /// A "Rust library" file.
    #[serde(rename = "rlib")]
    Rlib,
    /// A dynamic Rust library.
    #[serde(rename = "dylib")]
    Dylib,
    /// A dynamic system library
    #[serde(rename = "cdylib")]
    Cdylib,
    /// A static system library.
    #[serde(rename = "staticlib")]
    Staticlib,
    /// A procedural macro.
    #[serde(rename = "proc-macro")]
    ProcMacro,
    /// An example.
    #[serde(rename = "example")]
    Example,
    /// An integration test.
    #[serde(rename = "test")]
    Test,
    /// A benchmark.
    #[serde(rename = "bench")]
    Bench,
    /// A build script.
    #[serde(rename = "custom-build")]
    CustomBuild,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
/// The rust edition
pub enum Edition {
    /// Edition 2015
    #[serde(rename = "2015")]
    E2015,
    /// Edition 2018
    #[serde(rename = "2018")]
    E2018,
    /// Edition 2021
    #[serde(rename = "2021")]
    E2021,
}

impl Debug for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::E2015 => write!(f, "2015"),
            Self::E2018 => write!(f, "2018"),
            Self::E2021 => write!(f, "2021"),
        }
    }
}

impl Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::E2015 => write!(f, "2015"),
            Self::E2018 => write!(f, "2018"),
            Self::E2021 => write!(f, "2021"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resolve {}
