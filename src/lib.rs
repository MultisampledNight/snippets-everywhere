use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use serde::{Deserialize, Serialize};

pub mod backends;
pub mod ui;

pub fn run() -> Result<()> {
    let backends = backends::all();

    let cmdline = ui::cmdline(&backends);
    let BackendSelection { input, outputs } = BackendSelection::from_matches(cmdline, &backends)?;

    let input_file = fs::read_to_string(&input.path)
        .with_context(|| format!("error reading input for backend `{}`", input.path.display()))?;
    let ir = input.backend.deserialize(&input_file)?;

    for (path, backend) in outputs.mapping {
        let repr = backend.serialize(&ir)?;
        fs::write(&path, repr)
            .with_context(|| format!("error writing output for backend `{}`", path.display()))?;
    }

    Ok(())
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
    fn deserialize(&self, input: &str) -> Result<SnippetFile>;

    /// Tries writing _the IR:tm:_ into a string.
    ///
    /// # Panics
    ///
    /// Panics if the backend doesn't actually support serializing. Note to the implementor:
    /// Don't forget to also implement [`Backend::name_out`] to return [`None`] in that case.
    fn serialize(&self, snippets: &SnippetFile) -> Result<String>;

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
    /// # Errors
    ///
    /// Returns an error if multiple output backends point to the same file.
    ///
    /// # Panics
    ///
    /// May exhibit very unexpected behavior, including panics, if the given matches don't have
    /// exactly one input, or no outputs, but it will not do funny unsafe stuff.
    pub fn from_matches(
        matches: ArgMatches,
        registered: &'backends [Box<dyn Backend>],
    ) -> Result<Self> {
        let input = registered
            .iter()
            .find_map(|backend| {
                backend
                    .name_in()
                    .and_then(|name| matches.get_one(&name))
                    .map(|path: &PathBuf| Source {
                        backend: backend.as_ref(),
                        path: path.clone(),
                    })
            })
            .unwrap();

        let outputs = registered
            .iter()
            .filter_map(|backend| {
                backend
                    .name_out()
                    .and_then(|name| matches.get_one(&name))
                    .map(|path: &PathBuf| (path.clone(), backend.as_ref()))
            })
            // not ideal, but only (?) way to keep the iterator pattern while catching duplicates
            .try_fold(HashMap::new(), |mut map, (path, backend)| {
                if let Some(previous) = map.insert(path.clone(), backend) {
                    return Err(anyhow!(
                        "both backends `{}` and `{}` were given {} as path to write to, they'd overwrite each other. please use different paths for ",
                        previous.name(),
                        backend.name(),
                        path.display(),
                    ));
                }

                Ok(map)
            })?;

        Ok(Self {
            input,
            outputs: Targets { mapping: outputs },
        })
    }
}
