use std::fs;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use thiserror::Error;

mod parse;
mod serialize;
mod ui;

#[derive(Debug, Error)]
pub enum RunError {
    #[error("Could not read input files: {0}")]
    ReadInputError(#[from] anyhow::Error),

    #[error("Parsing failed: {0}")]
    Parse(#[from] parse::Error),
}

pub fn run() -> Result<(), RunError> {
    let cmdline = ui::cmdline();
    let input = InputKind::try_from(cmdline.input)?;

    let ir = parse::parse(input)?;
    dbg!(ir);

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SnippetFile {
    snippets: Vec<Snippet>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Snippet {
    trigger: String,
    description: Option<String>,
    replacement: String,
    options: Option<String>,
    priority: Option<i64>,
}

#[derive(Debug)]
pub enum InputKind {
    Ols(String),
    UltiSnips(String),
}

impl TryFrom<ui::Input> for InputKind {
    type Error = anyhow::Error;

    fn try_from(user_input: ui::Input) -> Result<Self, Self::Error> {
        // order doesn't matter since `cmdline` accepts only one kind of input -- at least atm
        if let Some(path) = user_input.ols_in {
            Ok(Self::Ols(fs::read_to_string(&path).with_context(|| {
                format!("could not read OLS snippet file at {}", path.display())
            })?))
        } else if let Some(path) = user_input.ultisnips_in {
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

pub enum OutputKind {
    Ols(SnippetFile),
    UltiSnips(SnippetFile),
}
