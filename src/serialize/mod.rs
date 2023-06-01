mod ols;

use thiserror::Error;

use crate::{BackendKind, SnippetFile};

pub fn serialize(backend: BackendKind, snippets: &SnippetFile) -> Result<String, Error> {
    let repr = match backend {
        BackendKind::Ols => ols::serialize(snippets)?,
        _ => todo!(),
    };

    Ok(repr)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("While serializing to Obsidian LaTeX suite's format: {0}")]
    Ols(#[from] ols::SerializeError),
    #[error("While serializing to UltiSnips' format: ")]
    UltiSnips(),
}
