use thiserror::Error;

use crate::SnippetFile;

pub fn serialize(snippets: &SnippetFile) -> Result<String, SerializeError> {
    // json5 also has a to_string function, but nothing for pretty printing
    serde_json::to_string_pretty(&snippets.snippets).map_err(Into::into)
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct SerializeError(#[from] serde_json::Error);
