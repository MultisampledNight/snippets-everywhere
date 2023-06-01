use std::{collections::HashMap, fs, io, path::PathBuf};

use anyhow::Context;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};

pub mod backends;
pub mod ui;

pub fn run() -> Result<(), anyhow::Error> {
    let backends = backends::all();

    let cmdline = ui::cmdline(&backends);
    let backend_selection = BackendSelection::from_matches(cmdline, &backends);

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

pub trait Backend {
    /// Tries parsing the given input into _the IR:tm:_.
    ///
    /// # Panics
    ///
    /// Panics if the backend doesn't actually support deserializing.
    fn deserialize(&self, input: String) -> Result<SnippetFile, anyhow::Error>;
    fn supports_deserialization(&self) -> bool;

    /// Tries writing _the IR:tm:_ into a string.
    ///
    /// # Panics
    ///
    /// Panics if the backend doesn't actually support deserializing.
    fn serialize(&self, snippets: SnippetFile) -> Result<String, anyhow::Error>;
    fn supports_serialization(&self) -> bool;

    /// The name of this backend, ideally an all-lowercase, short identifier.
    fn name(&self) -> &'static str;
}

pub struct BackendSelection<'backends> {
    pub input: Source<'backends>,
    pub outputs: Vec<Target<'backends>>,
}

pub struct Source<'backend> {
    pub backend: &'backend dyn Backend,
    pub path: PathBuf,
}

pub struct Target<'backend> {
    pub backend: &'backend dyn Backend,
    pub path: PathBuf,
}

impl<'backends> BackendSelection<'backends> {
    /// Constructs a selection of backends to parse from/convert to according to clap.
    ///
    /// **Note**: May exhibit very unexpected behavior if the given matches don't have exactly
    /// one input, or no outputs, but it will not panic or do funny unsafe stuff.
    pub fn from_matches(matches: ArgMatches, registered: &'backends [Box<dyn Backend>]) -> Self {
        todo!()
    }
}
