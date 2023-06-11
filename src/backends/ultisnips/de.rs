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
    // TODO: `extends` command, maybe not even necessary
    let quote_end = just('X') // placeholder char -- doesn't matter, will be immediately replaced
        .configure(|cfg, first_ch| cfg.seq(*first_ch));
    let quoted_trigger = any().then_with_ctx(
        any()
            // the whitespace check is necessary to find out
            // if that's really been at the end of the word and not in-between
            // (yeah, that's actually not what UltiSnips does, but UltiSnips does RTL
            // parsing anyway, we're merely trying to emulate it)
            .and_is(quote_end.then(text::whitespace().at_least(1)).not())
            .and_is(text::newline().not())
            .repeated()
            .collect::<String>()
            .then_ignore(quote_end),
    );

    let unquoted_trigger = any()
        .and_is(text::whitespace().at_least(1).not())
        .repeated()
        .collect::<String>();

    // :h UltiSnips-basic-syntax doesn't give any more explanation on escaping or the like, apart
    // from the description being always quoted. so we'll do exactly that
    let description = any()
        .and_is(just('"').not())
        .repeated()
        .collect::<String>()
        .padded_by(just('"'));

    let options = just('A').map(|ch| ch.to_string());

    let content = any::<_, extra::Err<Simple<char>>>()
        .and_is(just('\n').then(text::keyword("endsnippet")).not())
        .repeated()
        .collect::<String>()
        .then_ignore(just('\n').then(text::keyword("endsnippet")));

    let space = one_of(" \t").repeated().at_least(1);
    let signature = text::keyword("snippet")
        .then(space)
        .ignore_then(quoted_trigger.or(unquoted_trigger))
        .then(
            space
                .ignore_then(description.then(space.ignore_then(options).or_not()))
                .or_not(),
        );

    let snippet = signature.then_ignore(just('\n')).then(content).map(
        |((trigger, maybe_more), replacement)| {
            let (description, options) = match maybe_more {
                Some((desc, None)) => (Some(desc), None),
                Some((desc, Some(opts))) => (Some(desc), Some(opts)),
                _ => (None, None),
            };

            Snippet {
                trigger,
                replacement,
                description,
                options,
                priority: None,
            }
        },
    );

    let comment =
        just::<_, _, extra::Err<Simple<char>>>('#').then(any().and_is(just('\n').not()).repeated());

    let everything_ignored = choice((text::whitespace().at_least(1), comment.ignored())).repeated();

    snippet
        .padded_by(everything_ignored)
        .repeated()
        .collect::<Vec<_>>()
        .map(|snippets| SnippetFile { snippets })
}
