mod ser;

use crate::{Backend, SnippetFile};

/// Backend for de- and serializing [UltiSnips] snippet files.
///
/// [UltiSnips]: https://github.com/SirVer/ultisnips
#[derive(Debug)]
pub struct UltiSnips;

impl Backend for UltiSnips {
    fn name(&self) -> &'static str {
        "ultisnips"
    }

    fn name_in(&self) -> Option<String> {
        // doesn't support parsing... yet
        None
    }

    fn deserialize(&self, _input: &str) -> anyhow::Result<SnippetFile> {
        todo!()
    }

    fn serialize(&self, snippets: &SnippetFile) -> anyhow::Result<String> {
        ser::serialize(snippets)
    }
}
