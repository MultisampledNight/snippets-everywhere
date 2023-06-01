mod ols;

use thiserror::Error;

use crate::{InputDesc, SnippetFile};

pub fn parse(input: InputDesc) -> Result<SnippetFile, Error> {
    match input {
        InputDesc::Ols(unparsed) => ols::parse(unparsed).map_err(Into::into),
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
