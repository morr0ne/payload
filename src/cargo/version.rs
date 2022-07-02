use std::str::{FromStr, Lines};
use target_lexicon::Triple;
use time::{format_description::well_known::Iso8601, Date};

use super::{ParsingError, Result};

/// Parsed output of running "cargo --version --verbose".
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    /// Version of this release of cargo, E.g. "1.62.0".
    pub release: String,
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
        /// Finds the next line that matches the prefix
        fn strip_line(lines: &mut Lines, prefix: &'static str) -> Option<String> {
            lines
                .find_map(|line| line.strip_prefix(prefix))
                .map(|line| line.to_string())
        }

        let mut lines = s.lines();

        // This is safe since it upholds the above safety requirements
        Ok(Version {
            release: strip_line(&mut lines, "release: ").ok_or(ParsingError::Version("release"))?,
            commit_hash: strip_line(&mut lines, "commit-hash: "),
            commit_date: strip_line(&mut lines, "commit-date: ")
                .map(|commit_date| Date::parse(&commit_date, &Iso8601::DEFAULT).unwrap()), // TODO: This is very inefficient.
            host: strip_line(&mut lines, "host: ")
                .ok_or(ParsingError::Version("host"))?
                .parse()
                .unwrap(), // TODO: This is very inefficient.
            libgit2: strip_line(&mut lines, "libgit2: ").ok_or(ParsingError::Version("libgit2"))?,
            libcurl: strip_line(&mut lines, "libcurl: ").ok_or(ParsingError::Version("libcurl"))?,
            os: strip_line(&mut lines, "os: ").ok_or(ParsingError::Version("os"))?,
        })
    }
}
