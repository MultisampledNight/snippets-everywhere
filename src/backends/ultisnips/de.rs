use chumsky::prelude::*;
use thiserror::Error;

use crate::SnippetFile;

pub fn deserialize(input: &str) -> anyhow::Result<SnippetFile> {
    parser().parse(input).map_err(ParseError).map_err(Into::into)
}

#[derive(Debug, Error)]
#[error(
    "error(s) parsing UltiSnips snippet file:\n{}",
    .0.iter().flat_map(|err| {
        err
            .to_string()
            .chars()
            .collect::<Vec<_>>()
            .into_iter()
    }).collect::<String>()
)]
struct ParseError(Vec<Simple<char>>);

fn parser() -> impl Parser<char, SnippetFile, Error = Simple<char>> {
    todo()
}
