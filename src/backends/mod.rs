//! Semi-dynamic backend system
//!
//! A backend is a struct implementing [`Backend`], having a name, offering deserialization
//! (input) abilities through [`Backend::deserialize`] and serialization (output) abilities
//! through [`Backend::serialize`], using [`SnippetFile`] as the linking part between them.
//!
//! In order to create a new backend, create a struct implementing [`Backend`] and add it to the
//! [`all`] method in this module. That's all what's needed, it'll appear on the CLI automatically.
//!
//! If a backend happens to support only deserialization or only serialization (or neither, in
//! which case it's inaccessible though), it's also possible to override [`Backend::name_in`] and
//! [`Backend::name_out`] respectively and make them return [`None`] instead.

mod ols;
mod ultisnips;

use anyhow::Result;
pub use ols::Ols;
pub use ultisnips::UltiSnips;

use crate::SnippetFile;

/// All registered backends.
pub fn all() -> Vec<Box<dyn Backend>> {
    vec![Box::new(Ols), Box::new(UltiSnips)]
}

/// Offers communication to and from a file format. See the module-level docs for details.
pub trait Backend: std::fmt::Debug {
    /// Tries parsing the given input into _the IR_.
    ///
    /// # Panics
    ///
    /// Panics if the backend doesn't actually support deserializing. Note to the implementor:
    /// Don't forget to also implement [`Backend::name_in`] to return [`None`] in that case.
    fn deserialize(&self, input: &str) -> Result<SnippetFile>;

    /// Tries writing _the IR_ into a string.
    ///
    /// # Panics
    ///
    /// Panics if the backend doesn't actually support serializing. Note to the implementor:
    /// Don't forget to also implement [`Backend::name_out`] to return [`None`] in that case.
    fn serialize(&self, snippets: &SnippetFile) -> Result<String>;

    /// The name of this backend, ideally an all-lowercase, short identifier.
    fn name(&self) -> &'static str;

    /// Returns the input argument name for this backend. Returns [`None`] if this backend
    /// doesn't support deserialization.
    fn name_in(&self) -> Option<String> {
        Some(format!("{}-in", self.name()))
    }

    /// Returns the input argument name for this backend. Returns [`None`] if this backend
    /// doesn't support serialization.
    fn name_out(&self) -> Option<String> {
        Some(format!("{}-out", self.name()))
    }
}
