use anyhow::Context;

use crate::{Backend, Snippet, SnippetFile};

/// Backend for de- and serializing [Obsidian LaTeX suite] snippet files.
///
/// [Obsidian LaTeX suite]: https://github.com/artisticat1/obsidian-latex-suite
#[derive(Debug)]
pub struct Ols;

impl Backend for Ols {
    fn name(&self) -> &'static str {
        "ols"
    }

    fn deserialize(&self, input: &str) -> Result<SnippetFile, anyhow::Error> {
        let snippets: Vec<Snippet> =
            json5::from_str(input).context("error while parsing OLS snippets")?;

        Ok(SnippetFile { snippets })
    }

    fn serialize(&self, snippets: &SnippetFile) -> Result<String, anyhow::Error> {
        json5::to_string(&snippets.snippets).map_err(Into::into)
    }
}
