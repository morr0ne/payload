use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use target_lexicon::Triple;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnitGraph {
    /// Version of the JSON output structure.
    /// If any backwards incompatible changes are made, this value will be increased.
    version: usize,
    /// Array of all build units.
    pub units: Vec<Unit>,
    /// Array of indices in the "units" array that are the "roots" of the dependency graph.
    pub roots: Vec<usize>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Unit {
    /// An opaque string which indicates the package.
    pub pkg_id: String,
    /// The Cargo target
    pub target: Target,
    /// The profile settings for this unit.
    /// These values may not match the profile defined in the manifest.
    /// Units can use modified profile settings. For example, the "panic"
    /// setting can be overridden for tests to force it to "unwind".
    pub profile: Profile,
    /// Which platform this target is being built for.
    /// A value of `None` indicates it is for the host.
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub platform: Option<Triple>,
    /// The "mode" for this unit.
    pub mode: Mode,
    /// Array of features enabled on this unit.
    pub features: Vec<String>,
    /// Whether or not this is a standard-library unit,
    /// part of the unstable build-std feature
    #[serde(default)]
    pub is_std: bool,
    /// Array of dependencies of this unit.
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Target {
    pub kind: Vec<TargetKind>,
    pub crate_types: Vec<String>, // TODO: Not sure if it should be TargetKind or anotther enum.
    pub name: String,
    pub src_path: PathBuf,
    pub edition: Edition,
    #[serde(rename = "required-features")]
    pub required_features: Vec<String>,
    pub doc: bool,
    pub doctest: bool,
    /// Whether or not this target should be built and run with `--test`
    pub test: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetKind {
    #[serde(rename = "bin")]
    Bin,
    #[serde(rename = "lib")]
    Lib,
    #[serde(rename = "rlib")]
    Rlib,
    #[serde(rename = "dylib")]
    Dylib,
    #[serde(rename = "cdylib")]
    Cdylib,
    #[serde(rename = "staticlib")]
    Staticlib,
    #[serde(rename = "proc-macro")]
    ProcMacro,
    #[serde(rename = "example")]
    Example,
    #[serde(rename = "integration")]
    Integration,
    #[serde(rename = "benchmark")]
    Benchmark,
    #[serde(rename = "custom-build")]
    CustomBuild,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Profile {
    /// The profile name these settings are derived from.
    pub name: String, // TODO: could be an enum.
    /// The optimization level.
    pub opt_level: String, // TODO: could be an enum.
    /// The LTO setting.
    pub lto: String, // TODO: Almost definitely could be an enum
    /// The codegen units as an integer.
    /// `None` if it should use the compiler's default.
    pub codegen_units: Option<u32>,
    /// The debug information level as an integer.
    /// `None` if it should use the compiler's default (0).
    pub debuginfo: Option<u32>,
    /// Whether or not debug-assertions are enabled.
    pub debug_assertions: bool,
    /// Whether or not overflow-checks are enabled.
    pub overflow_checks: bool,
    /// Whether or not incremental is enabled.
    pub rpath: bool,
    /// Whether or not incremental is enabled.
    pub incremental: bool,
    /// The panic strategy.
    pub panic: PanicStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PanicStrategy {
    #[serde(rename = "unwind")]
    Unwind,
    #[serde(rename = "abort")]
    Abort,
}

/// The "mode" of a unit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Mode {
    /// Build using `rustc` as a test.
    #[serde(rename = "test")]
    Test,
    /// Build using `rustc`.
    #[serde(rename = "build")]
    Build,
    /// Build using `rustc` in "check" mode.
    #[serde(rename = "check")]
    Check,
    /// Build using `rustdoc`.
    #[serde(rename = "doc")]
    Doc,
    /// Test using `rustdoc`.
    #[serde(rename = "doctest")]
    Doctest,
    /// Represents the execution of a build script.
    #[serde(rename = "run-custom-build")]
    RunCustomBuild,
}

/// Array of dependencies of a unit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dependency {
    /// Index in the "units" array for the dependency.
    pub index: usize,
    /// The name that this dependency will be referred as.
    pub extern_crate_name: String,
    /// Whether or not this dependency is "public",
    /// part of the unstable public-dependency feature.
    /// If `None` the public-dependency feature is not enabled.
    pub public: Option<bool>,
    /// Whether or not this dependency is injected into the prelude,
    /// currently used by the build-std feature.
    #[serde(default)]
    pub noprelude: bool,
}
