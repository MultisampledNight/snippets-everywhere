use anyhow::Result;
use itertools::Itertools;
use thiserror::Error;

use crate::{Snippet, SnippetFile};

pub fn deserialize(input: &str) -> Result<SnippetFile> {
    // TODO: `priority n` commands
    // TODO: `extends` command, maybe not even necessary

    let mut snippets = Vec::new();
    let mut current_priority = 0;

    // external since it's also used inside the loop itself
    let mut lines_iter = input.lines().peekable();

    while let Some(line) = lines_iter.next() {
        let line = line.trim();

        if line.starts_with('#') {
            continue;
        }

        match line.split_whitespace().next() {
            None => continue, // will have just been whitespace or completely empty
            Some("snippet") => {
                let mut relevant_lines = vec![line.to_string()];

                relevant_lines.extend(
                    lines_iter
                        .peeking_take_while(|line| line.trim() != "endsnippet")
                        .map(|line| line.to_string()),
                );
                relevant_lines.push(lines_iter.next().unwrap().to_string());

                let snippet = parse_snippet(&relevant_lines, &mut current_priority)?;
                snippets.push(snippet);
            }
            Some(unknown) => {
                return Err(ParseError::UnknownDirective {
                    directive: unknown.to_string(),
                }.into())
            }
        }
    }

    Ok(SnippetFile { snippets })
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("unknown directive: `{directive}`")]
    UnknownDirective { directive: String },
}

fn parse_snippet(lines: &[String], current_priority: &mut i64) -> Result<Snippet> {
    dbg!(lines);
    todo!()
}
