use std::collections::HashSet;

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

    fn deserialize(&self, input: String) -> Result<SnippetFile, anyhow::Error> {
        let snippets: Vec<Snippet> =
            json5::from_str(&input).context("Error while parsing OLS snippets")?;

        // filter the mode specifiers since UltiSnips doesn't know them
        let disallowed: HashSet<_> = "tmc".chars().collect();
        let snippets = snippets
            .into_iter()
            .map(|snippet| Snippet {
                options: snippet
                    .options
                    .map(|opts| { opts.chars().filter(|c| !disallowed.contains(c)) }.collect()),
                ..snippet
            })
            .collect();

        Ok(SnippetFile { snippets })
    }

    fn supports_deserialization(&self) -> bool {
        true
    }

    fn serialize(&self, snippets: SnippetFile) -> Result<String, anyhow::Error> {
        // json5 also has a to_string function, but nothing for pretty printing
        // not sure if pretty printing is actually needed tbh, likely it's only to be read by
        // the program anyway
        serde_json::to_string_pretty(&snippets.snippets).map_err(Into::into)
    }

    fn supports_serialization(&self) -> bool {
        true
    }
}
