use thiserror::Error;

use crate::SnippetFile;

pub fn serialize(data: &SnippetFile) -> Result<String, SerializeError> {
    json5::to_string(&data.snippets).map_err(Into::into)
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct SerializeError(#[from] json5::Error);
