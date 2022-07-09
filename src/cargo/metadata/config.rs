use std::{path::PathBuf, process::Command};
use target_lexicon::Triple;

#[derive(Debug, Default)]
pub struct MetadataConfig {
    features: Option<Features>,
    filter_platform: Option<Triple>,
    manifest_path: Option<PathBuf>,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Features {
    AllFeatures,
    NoDefaultFeatures,
    SomeFeatures(Vec<String>),
}

impl MetadataConfig {
    pub fn push_args(&self, command: &mut Command) {
        if let Some(features) = &self.features {
            match features {
                Features::AllFeatures => {
                    command.arg("--all-features");
                }
                Features::NoDefaultFeatures => {
                    command.arg("--no-default-features");
                }
                Features::SomeFeatures(features) => {
                    command.arg("--features").arg(features.join(","));
                }
            }
        }

        if let Some(filter_platform) = &self.filter_platform {
            command
                .arg("--filter-platform")
                .arg(&filter_platform.to_string());
        }

        if let Some(manifest_path) = &self.manifest_path {
            command.arg("--manifest-path").arg(manifest_path);
        }
    }
}
