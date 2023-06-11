use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use backends::Backend;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};

pub mod backends;
pub mod ui;

pub fn run() -> Result<()> {
    let backends = backends::all();

    let cmdline = ui::cmdline(&backends);
    let BackendSelection { input, outputs } = BackendSelection::from_matches(cmdline, &backends)?;

    let input_file = fs::read_to_string(&input.path).with_context(|| {
        format!(
            "error reading input for backend `{}` at path {}",
            input.backend.name(),
            input.path.display()
        )
    })?;
    let ir = input.backend.deserialize(&input_file)?;

    for (path, backend) in outputs.mapping {
        let repr = backend.serialize(&ir)?;
        fs::write(&path, repr).with_context(|| {
            format!(
                "error writing output for backend `{}` at path {}",
                backend.name(),
                path.display()
            )
        })?;
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

    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<i64>,
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
                map
                    .insert(path.clone(), backend)
                    .map_or(Ok(map), |previous| Err(anyhow!(
                        "both backends `{}` and `{}` were given {} as path to write to, they'd overwrite each other. please use different paths for ",
                        previous.name(),
                        backend.name(),
                        path.display(),
                    )))
            })?;

        Ok(Self {
            input,
            outputs: Targets { mapping: outputs },
        })
    }
}
