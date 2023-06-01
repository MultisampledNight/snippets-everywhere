mod ols;

use thiserror::Error;

use crate::{InputKind, SnippetFile};

pub fn parse(input: InputKind) -> Result<SnippetFile, Error> {
    match input {
        InputKind::Ols(unparsed) => ols::parse(unparsed).map_err(Into::into),
        _ => todo!(),
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("In snippets of Obsidian LaTeX suite: {0}")]
    Ols(#[from] ols::ParseError),
    #[error("In snippets of UltiSnips: ")]
    UltiSnips(),
}
