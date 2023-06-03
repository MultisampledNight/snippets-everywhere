use chumsky::prelude::*;
use thiserror::Error;

use crate::{SnippetFile, Snippet};

pub fn deserialize(input: &str) -> anyhow::Result<SnippetFile> {
    parser().parse(input).map_err(ParseError).map_err(Into::into)
}

#[derive(Debug, Error)]
#[error(
    "error(s) parsing UltiSnips snippet file:\n{}",
    .0.into_iter().map(|err| format!("{err}").chars()).flatten().collect::<String>()
)]
struct ParseError(Vec<Simple<char>>);

fn parser() -> impl Parser<char, SnippetFile, Error = Simple<char>> {
    todo!()
}
