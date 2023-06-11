use crate::Snippet;

use super::*;

#[test]
fn deserialize_multiple() {
    let input = r#"
snippet written "" Aw
replaced
endsnippet

snippet a "" A
b
endsnippet

snippet written
conflict
endsnippet
    "#;

    let ir = UltiSnips.deserialize(input).unwrap();
    assert_eq!(
        ir,
        SnippetFile {
            snippets: vec![
                Snippet {
                    trigger: "written".to_string(),
                    replacement: "replaced".to_string(),
                    description: Some(String::new()),
                    options: Some("Aw".to_string()),
                    ..Default::default()
                },
                Snippet {
                    trigger: "a".to_string(),
                    replacement: "b".to_string(),
                    description: Some(String::new()),
                    options: Some("A".to_string()),
                    ..Default::default()
                },
                Snippet {
                    trigger: "written".to_string(),
                    replacement: "conflict".to_string(),
                    ..Default::default()
                },
            ]
        }
    );
}
