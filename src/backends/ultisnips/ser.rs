use std::{collections::HashSet, fmt::Write};

use thiserror::Error;

use crate::SnippetFile;

pub fn serialize(snippets: &SnippetFile) -> anyhow::Result<String> {
        let mut output = String::new();
        let mut last_priority = 0;

        for snippet in &snippets.snippets {
            // very much recommended to look at :h UltiSnips-basic-syntax while reading this
            write_and_update_priority(&mut output, &mut last_priority, snippet.priority);

            write!(output, "snippet").unwrap();

            write_trigger(&mut output, &snippet.trigger)?;
            write_description_and_options(
                &mut output,
                snippet.description.as_deref(),
                snippet.options.as_deref(),
            );

            writeln!(output, "\n{}", snippet.replacement).unwrap();
            writeln!(output, "endsnippet\n").unwrap();
        }

        Ok(output)
    }


fn write_and_update_priority(output: &mut String, last_priority: &mut i64, priority: Option<i64>) {
    let priority = priority.unwrap_or(0);
    if priority == *last_priority {
        return; // last priority still applies
    }

    writeln!(output, "priority {priority}").unwrap();
}

fn write_trigger(output: &mut String, trigger: &str) -> Result<(), SerializeError> {
    if !trigger.contains(' ') {
        // all fine, no quotes needed
        write!(output, " {}", trigger).unwrap();
        return Ok(());
    }

    // uuuuuh okay we need to quote in some way, let's find something sane
    // ...looking at _handle_snippets_or_global in the ultisnips source code,
    // actually any character is fine
    let possible_quotes = "\"'#!?%|/^~=&:,$&¢αβγδμ´¹²³ඞ";
    let chars_in_trigger: HashSet<_> = trigger.chars().collect();

    if let Some(quote) = possible_quotes
        .chars()
        .find(|candidate| !chars_in_trigger.contains(candidate))
    {
        write!(output, " {}{}{}", quote, trigger, quote).unwrap();
        return Ok(());
    };

    Err(SerializeError::CouldNotQuote {
        trigger: trigger.to_string(),
    })
}

fn write_description_and_options(
    output: &mut String,
    description: Option<&str>,
    options: Option<&str>,
) {
    if description.is_some() || options.is_some() {
        write!(output, " \"{}\"", description.unwrap_or(""),).unwrap();

        if let Some(options) = options {
            // filter the mode specifiers since UltiSnips doesn't know them
            let disallowed: HashSet<_> = "tmc".chars().collect();
            let options: String = options
                .chars()
                .filter(|opt| !disallowed.contains(opt))
                .collect();

            write!(output, " {options}").unwrap();
        }
    }
}

#[derive(Debug, Error)]
pub enum SerializeError {
    #[error("The trigger {trigger} has so many special characters that I'm unable to find a proper quote character, in order to insert it properly. Please consider using a more sane trigger, or open a bug report with your trigger and usecase.")]
    CouldNotQuote { trigger: String },
}
