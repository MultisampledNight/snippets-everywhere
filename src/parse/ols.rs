use thiserror::Error;

use crate::SnippetFile;

pub fn parse(input: String) -> Result<SnippetFile, ParseError> {
    Ok(SnippetFile {
        snippets: json5::from_str(&input)?,
    })
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] json5::Error);
