use semver::Version as SemverVersion;
use std::str::FromStr;
use target_lexicon::Triple;
use time::{format_description::well_known::Iso8601, Date};

use super::{ParsingError, Result};

/// Parsed output of running "cargo --version --verbose".
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    /// Version of this release of cargo, E.g. "1.62.0".
    pub release: SemverVersion,
    /// The sha1 hash of the latest commit when this version of cargo was published.
    ///
    /// May be missing if not built from git.
    pub commit_hash: Option<String>,
    /// The date of the latest commit when this version of cargo was published.
    ///
    /// May be missing if not built from git.
    pub commit_date: Option<Date>,
    /// The host target triple, E.g. "x86_64-unknown-linux-gnu".
    pub host: Triple,
    /// The version of the bundled libgit2.
    pub libgit2: String,
    /// The version of the bundled libcurl.
    pub libcurl: String,
    /// The current operating system, E.g. "Arch Linux Rolling Release \[64-bit\]".
    pub os: String,
}

impl FromStr for Version {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Search for a line starting with "release: ", if not found returns an error otherwise it parses it as a semver::Version.
        let release = lines
            .find_map(|line| line.strip_prefix("release: "))
            .ok_or(ParsingError::Version("release"))?
            .parse()?;

        // Search for a line starting with "commit-hash: " and returns it still wrapped in an option.
        let commit_hash = lines
            .find_map(|line| line.strip_prefix("commit-hash: "))
            .map(|line| line.to_string());

        // Search for a line starting with "commit-date: ", parses it as a time::Date and returns it still wrapped in an option.
        let commit_date = lines
            .find_map(|line| line.strip_prefix("commit-date: "))
            .map(|line| line.to_string())
            .map(|commit_date| Date::parse(&commit_date, &Iso8601::DEFAULT).unwrap());

        // Search for a line starting with "host: ", if not found returns an error otherwise it parses it as a target_lexicon::Triple.
        let host = lines
            .find_map(|line| line.strip_prefix("host: "))
            .ok_or(ParsingError::Version("host"))?
            .parse()?;

        // Search for a line starting with "libgit2: ", if not found returns an error otherwise returns it as a String.
        let libgit2 = lines
            .find_map(|line| line.strip_prefix("libgit2: "))
            .map(|line| line.to_string())
            .ok_or(ParsingError::Version("libgit2"))?;

        // Search for a line starting with "libcurl: ", if not found returns an error otherwise returns it as a String.
        let libcurl = lines
            .find_map(|line| line.strip_prefix("libcurl: "))
            .map(|line| line.to_string())
            .ok_or(ParsingError::Version("libcurl"))?;

        // Search for a line starting with "os: ", if not found returns an error otherwise returns it as a String.
        let os = lines
            .find_map(|line| line.strip_prefix("os: "))
            .map(|line| line.to_string())
            .ok_or(ParsingError::Version("os"))?;

        Ok(Version {
            release,
            commit_hash,
            commit_date,
            host,
            libgit2,
            libcurl,
            os,
        })
    }
}
