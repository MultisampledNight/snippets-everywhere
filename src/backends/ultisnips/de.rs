use logos::Logos;
use thiserror::Error;

use crate::SnippetFile;

pub fn deserialize(input: &str) -> anyhow::Result<SnippetFile> {
    let lex = Token::lexer(input);
    let _ = dbg!(lex.collect::<Vec<Result<Token, ()>>>());

    todo!()
}

#[derive(Debug, Error)]
#[error("error(s) parsing UltiSnips snippet file:\n{0:?}")]
struct ParseError(());

#[derive(Logos, Clone, Debug, PartialEq, Eq)]
enum Token<'a> {
    #[token("snippet")]
    SnippetStart,

    #[regex("\\S+")]
    Identifier(&'a str),

    #[token("endsnippet")]
    SnippetEnd,

    #[token("\n")]
    Newline,

    #[regex(r"([ \t]+|#.*\n)", logos::skip)]
    Whitespace,
}
