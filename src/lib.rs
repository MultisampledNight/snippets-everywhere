use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};

pub mod backends;
pub mod ui;

pub fn run() -> Result<()> {
    let backends = backends::all();

    let cmdline = ui::cmdline(&backends);
    let backend_selection = BackendSelection::from_matches(cmdline, &backends);
    dbg!(backend_selection);

    todo!()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnippetFile {
    snippets: Vec<Snippet>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Snippet {
    trigger: String,
    replacement: String,
    options: Option<String>,
    description: Option<String>,
    priority: Option<i64>,
}

pub trait Backend: std::fmt::Debug {
    /// Tries parsing the given input into _the IR:tm:_.
    ///
    /// # Panics
    ///
    /// Panics if the backend doesn't actually support deserializing. Note to the implementor:
    /// Don't forget to also implement [`Backend::name_in`] to return [`None`] in that case.
    fn deserialize(&self, input: String) -> Result<SnippetFile>;

    /// Tries writing _the IR:tm:_ into a string.
    ///
    /// # Panics
    ///
    /// Panics if the backend doesn't actually support serializing. Note to the implementor:
    /// Don't forget to also implement [`Backend::name_out`] to return [`None`] in that case.
    fn serialize(&self, snippets: SnippetFile) -> Result<String>;

    /// The name of this backend, ideally an all-lowercase, short identifier.
    fn name(&self) -> &'static str;

    /// Returns the input argument name for this backend. Returns [`None`] if this backend
    /// doesn't support deserialization.
    fn name_in(&self) -> Option<String> {
        Some(format!("{}-in", self.name()))
    }

    /// Returns the input argument name for this backend. Returns [`None`] if this backend
    /// doesn't support serialization.
    fn name_out(&self) -> Option<String> {
        Some(format!("{}-out", self.name()))
    }
}

#[derive(Debug)]
pub struct BackendSelection<'backends> {
    pub input: Source<'backends>,
    pub outputs: Targets<'backends>,
}

#[derive(Debug)]
pub struct Source<'backend> {
    pub backend: &'backend dyn Backend,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Targets<'backends> {
    pub mapping: HashMap<PathBuf, &'backends dyn Backend>,
}

impl<'backends> BackendSelection<'backends> {
    /// Constructs a selection of backends to parse from/convert to according to clap.
    ///
    /// **Note**: May exhibit very unexpected behavior if the given matches don't have exactly
    /// one input, or no outputs, but it will not panic or do funny unsafe stuff.
    ///
    /// # Errors
    ///
    /// Returns an error if multiple output backends point to the same file.
    pub fn from_matches(
        matches: ArgMatches,
        registered: &'backends [Box<dyn Backend>],
    ) -> Result<Self> {
        let input = registered.iter().find(|backend| todo!());

        todo!()
    }
}
