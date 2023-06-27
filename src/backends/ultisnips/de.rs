use std::num::ParseIntError;

use anyhow::Result;
use itertools::Itertools;
use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

use crate::{Snippet, SnippetFile};

pub fn deserialize(input: &str) -> Result<SnippetFile> {
    // TODO: `extends` command, maybe not even necessary

    let mut snippets = Vec::new();
    let mut current_priority = None;

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

                let snippet = parse_snippet(&relevant_lines, current_priority)?;
                snippets.push(snippet);
            }
            Some("priority") => current_priority = Some(parse_priority(line)?),
            Some(unknown) => {
                return Err(ParseError::UnknownDirective {
                    directive: unknown.to_string(),
                }
                .into())
            }
        }
    }

    Ok(SnippetFile { snippets })
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("unknown directive: `{directive}`")]
    UnknownDirective { directive: String },
    #[error("found no trigger after `snippet`")]
    MissingSnippetTrigger,
    #[error("found no number after `priority` directive")]
    MissingPriorityNumber,
    #[error("tried to parse number in `{subject}` but failed: {err}")]
    ParsePriorityNumber { subject: String, err: ParseIntError },
}

fn parse_snippet(lines: &[String], priority: Option<i64>) -> Result<Snippet, ParseError> {
    // basically snippet/source/file/ulti_snips.py in the UltiSnips repo ported
    // first parsing the signature `snippet trigger [ description [ options ] ]`
    // remember: description and options are optional, trigger may be quoted weirdly
    let first_line = lines.get(0).expect("caller passing lines to parse");
    let mut parts: Vec<_> = first_line.split_whitespace().collect();

    let trigger;
    let mut description = None;
    let mut options = None;

    match parts.len() {
        0 => panic!("expected caller to not pass empty lines"),
        1 => return Err(ParseError::MissingSnippetTrigger),
        2 => {
            // `snippet trigger` (being unquoted)
            trigger = parts.last().unwrap().to_string();
        }
        _ => {
            // possibly description,
            // possibly description with option,
            // possibly just quoted trigger

            // are options there?
            if !parts.last().unwrap().ends_with('"') && parts[parts.len() - 2].ends_with('"') {
                options = parts.pop().map(|opts| opts.to_string());
            }

            // is a description there?
            if parts.last().unwrap().ends_with('"') {
                let _accumulated_desc = String::new();
                todo!()
            }

            // then everything remaining will be the trigger
            if parts.len() >= 2 {
                // quoted
                // actually according to :h UltiSnips-snippet-options, both single-word and
                // multi-word triggers can be quoted, but the code only implements the latter
                // (and checks for regex, too). so that's emulated here
                let quoted = parts[1..].iter().format(" ").to_string();
                let graphemes: Vec<_> = quoted.graphemes(true).collect();
                trigger = graphemes[1..graphemes.len() - 1].iter().copied().collect();
            } else {
                // unquoted
                trigger = parts[1].to_string();
            }
        }
    }

    let replacement = lines[1..lines.len() - 1].iter().format("\n").to_string();

    Ok(Snippet {
        trigger,
        replacement,
        options,
        description,
        priority,
    })
}

fn parse_priority(line: &str) -> Result<i64, ParseError> {
    line.split_whitespace()
        .nth(1)
        .ok_or(ParseError::MissingPriorityNumber)?
        .parse()
        .map_err(|err| ParseError::ParsePriorityNumber {
            subject: line.to_string(),
            err,
        })
}
