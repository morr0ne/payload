use std::{path::PathBuf, process::Command};
use target_lexicon::Triple;

#[derive(Debug, Default)]
pub struct MetadataConfig {
    pub features: Option<Features>,
    pub filter_platform: Option<Triple>,
    pub manifest_path: Option<PathBuf>,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Features {
    AllFeatures,
    NoDefaultFeatures,
    SomeFeatures(Vec<String>),
}
