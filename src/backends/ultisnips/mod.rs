#[cfg(test)]
mod tests;

mod de;
mod ser;

use crate::SnippetFile;

use super::Backend;

/// Backend for de- and serializing [UltiSnips] snippet files.
///
/// [UltiSnips]: https://github.com/SirVer/ultisnips
#[derive(Debug)]
pub struct UltiSnips;

impl Backend for UltiSnips {
    fn name(&self) -> &'static str {
        "ultisnips"
    }

    fn deserialize(&self, input: &str) -> anyhow::Result<SnippetFile> {
        de::deserialize(input)
    }

    fn serialize(&self, snippets: &SnippetFile) -> anyhow::Result<String> {
        ser::serialize(snippets)
    }
}
