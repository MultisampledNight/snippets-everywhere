use std::{collections::HashMap, fs, io, path::PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use thiserror::Error;

mod parse;
mod serialize;
mod ui;

pub fn run() -> Result<(), RunError> {
    let cmdline = ui::cmdline();

    let input = InputDesc::try_from(cmdline.input).map_err(RunError::ReadInputError)?;
    let ir = parse::parse(input)?;

    let output = OutputDesc::from(cmdline.output);
    for (backend, path) in output.targets {
        // fault tolerance? easy to implement, but I don't see the the use here
        let repr = serialize::serialize(backend, &ir)?;
        fs::write(&path, repr).map_err(|orig| RunError::WriteOutputError { path, orig })?;
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error("Could not read input files: {0}")]
    ReadInputError(anyhow::Error),

    #[error("Parsing failed: {0}")]
    Parse(#[from] parse::Error),

    #[error("Serialization failed: {0}")]
    Serialize(#[from] serialize::Error),

    #[error("Could not write output file {}: {orig}", .path.display())]
    WriteOutputError { path: PathBuf, orig: io::Error },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnippetFile {
    snippets: Vec<Snippet>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Snippet {
    trigger: String,
    description: Option<String>,
    replacement: String,
    options: Option<String>,
    priority: Option<i64>,
}

#[derive(Debug)]
pub enum InputDesc {
    Ols(String),
    UltiSnips(String),
}

impl TryFrom<ui::Input> for InputDesc {
    type Error = anyhow::Error;

    fn try_from(config: ui::Input) -> Result<Self, Self::Error> {
        // order doesn't matter since `cmdline` accepts only one kind of input -- at least atm
        if let Some(path) = config.ols_in {
            Ok(Self::Ols(fs::read_to_string(&path).with_context(|| {
                format!("could not read OLS snippet file at {}", path.display())
            })?))
        } else if let Some(path) = config.ultisnips_in {
            Ok(Self::UltiSnips(fs::read_to_string(&path).with_context(
                || {
                    format!(
                        "could not read UltiSnips snippet file at {}",
                        path.display()
                    )
                },
            )?))
        } else {
            unreachable!("either a new variant without handling has been added or the CLI parsing allows specifying no input")
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackendKind {
    Ols,
    UltiSnips,
}

pub struct OutputDesc {
    targets: HashMap<BackendKind, PathBuf>,
}

impl From<ui::Output> for OutputDesc {
    fn from(config: ui::Output) -> Self {
        Self {
            targets: [
                config.ols_out.map(|path| (BackendKind::Ols, path)),
                config
                    .ultisnips_out
                    .map(|path| (BackendKind::UltiSnips, path)),
            ]
            .into_iter()
            .flatten()
            .collect(),
        }
    }
}
