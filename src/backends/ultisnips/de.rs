use anyhow::anyhow;
use chumsky::prelude::*;
use thiserror::Error;

use crate::SnippetFile;

pub fn deserialize(input: &str) -> anyhow::Result<SnippetFile> {
    parser()
        .parse(input)
        .into_result()
        // directly converting the error to a string is... crude, but it works
        // can't directly return ParseError anyway since it contains a Simple with a lifetime
        // in a more serious project probably a custom/another error type would be better
        .map_err(|err| anyhow!(ParseError(err).to_string()))
        .map(|x| {
            dbg!(x);
            todo!()
        })
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
struct ParseError<'a>(Vec<Simple<'a, char>>);

fn parser<'a>() -> impl Parser<'a, &'a str, Vec<String>, extra::Err<Simple<'a, char>>> {
    // TODO: `priority n` commands
    let physical_whitespace = text::whitespace().at_least(1);

    // UltiSnips has interesting quote rules: if the first character is the same one as the last
    // one, the trigger is quoted with that char as quote character
    // the quote character can be _anything_ though, and quoting is not necessary
    let quote_end = just('X') // placeholder char -- doesn't matter, will be immediately replaced
        .configure(|cfg, first_ch| cfg.seq(*first_ch));
    let quoted_trigger = any()
        .then_with_ctx(
            any()
                .and_is(quote_end.not())
                .repeated()
                .collect::<String>()
                .then_ignore(quote_end),
        )
        .then_ignore(physical_whitespace);

    let unquoted_trigger = any()
        .and_is(physical_whitespace.not())
        .repeated()
        .collect::<String>()
        .then_ignore(physical_whitespace);

    let snippet = text::keyword("snippet")
        .then(physical_whitespace)
        .ignore_then(quoted_trigger.or(unquoted_trigger));

    let comment = just('#')
        .then(any().and_is(just('\n').not()).repeated())
        .then(text::whitespace().at_least(1));

    comment
        .padded()
        .repeated()
        .ignore_then(snippet)
        .repeated()
        .collect::<Vec<_>>()
        .then_ignore(any().repeated())
}
