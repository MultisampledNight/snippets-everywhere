use std::collections::HashSet;

use thiserror::Error;

use crate::{Snippet, SnippetFile};

pub fn parse(input: String) -> Result<SnippetFile, ParseError> {
    let snippets: Vec<Snippet> = json5::from_str(&input)?;

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

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] json5::Error);
