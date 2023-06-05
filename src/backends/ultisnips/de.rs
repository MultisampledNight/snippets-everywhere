use anyhow::anyhow;
use chumsky::prelude::*;
use thiserror::Error;

use crate::{Snippet, SnippetFile};

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

fn parser<'a>() -> impl Parser<'a, &'a str, SnippetFile, extra::Err<Simple<'a, char>>> {
    // TODO: `priority n` commands
    // UltiSnips has interesting quote rules: if the first character is the same one as the last
    // one, the trigger is quoted with that char as quote character
    // the quote character can be _anything_ though, and quoting is not necessary
    let quote_end = just('X') // placeholder char -- doesn't matter, will be immediately replaced
        .configure(|cfg, first_ch| cfg.seq(*first_ch));
    let quoted_trigger = any().then_with_ctx(
        any()
            // the whitespace check is necessary to find out
            // if that's really been at the end of the word and not in-between
            // (yeah, that's actually not what UltiSnips does, but UltiSnips does RTL
            // parsing anyway, we're merely trying to emulate it)
            .and_is(quote_end.then(text::whitespace().at_least(1)).not())
            .repeated()
            .collect::<String>()
            .then_ignore(quote_end),
    );

    let unquoted_trigger = any()
        .and_is(text::whitespace().at_least(1).not())
        .repeated()
        .collect::<String>();

    let description = todo();

    let options = todo();

    let end = just('\n').then(text::keyword("endsnippet"));

    let snippet = text::keyword("snippet")
        .then(text::whitespace().at_least(1))
        .ignore_then(quoted_trigger.or(unquoted_trigger))
        .then_ignore(just('\n'))
        .then(
            any()
                .and_is(end.clone().not())
                .repeated()
                .collect::<String>()
                .then_ignore(end),
        )
        .then(description.then(options.or_not()).or_not())
        .map(|((trigger, replacement), maybe_more)| {
            let (description, options) = match maybe_more {
                Some((desc, None)) => (desc, None),
                Some((desc, Some(opts))) => (desc, opts),
                _ => (None, None),
            };

            Snippet {
                trigger,
                replacement,
                description,
                options,
                priority: None,
            }
        });

    let comment = just('#')
        .then(any().and_is(just('\n').not()).repeated())
        .then(just('\n'));

    let everything_ignored = choice((text::whitespace().exactly(1), comment.ignored())).repeated();

    snippet
        .padded_by(everything_ignored)
        .repeated()
        .collect::<Vec<_>>()
        .map(|snippets| SnippetFile { snippets })
}
