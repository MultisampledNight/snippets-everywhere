use crate::Snippet;

use super::*;

#[test]
fn deserialize_unquoted() {
    let input = r#"
snippet written
wow
endsnippet
    "#;

    let ir = UltiSnips.deserialize(input).unwrap();
    assert_eq!(
        ir,
        SnippetFile {
            snippets: vec![Snippet {
                trigger: "written".to_string(),
                replacement: "wow".to_string(),
                ..Default::default()
            }]
        }
    );
}

#[test]
fn deserialize_quoted() {
    let input = r#"
snippet woah this is a long trigger wow
truly
endsnippet
    "#;

    let ir = UltiSnips.deserialize(input).unwrap();
    assert_eq!(
        ir,
        SnippetFile {
            snippets: vec![Snippet {
                // yes, this is intended
                trigger: "oah this is a long trigger wo".to_string(),
                replacement: "truly".to_string(),
                ..Default::default()
            }]
        }
    );
}

#[test]
fn deserialize_desc_and_opts() {
    let input = r#"
snippet written "" Aw
replaced
endsnippet
    "#;

    let ir = UltiSnips.deserialize(input).unwrap();
    assert_eq!(
        ir,
        SnippetFile {
            snippets: vec![Snippet {
                trigger: "written".to_string(),
                replacement: "replaced".to_string(),
                description: Some(String::new()),
                options: Some("Aw".to_string()),
                ..Default::default()
            }],
        },
    );
}

#[test]
fn deserialize_very_concise_with_desc() {
    let input = r#"
snippet a ""
b
endsnippet
    "#;

    let ir = UltiSnips.deserialize(input).unwrap();
    assert_eq!(
        ir,
        SnippetFile {
            snippets: vec![Snippet {
                trigger: "a".to_string(),
                replacement: "b".to_string(),
                description: Some(String::new()),
                ..Default::default()
            }],
        },
    )
}
