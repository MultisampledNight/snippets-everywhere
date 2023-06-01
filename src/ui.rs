use std::path::PathBuf;

use clap::{Args, Parser};

pub fn cmdline() -> Cmdline {
    Cmdline::parse()
}

#[derive(Parser, Debug)]
pub struct Cmdline {
    #[command(flatten)]
    pub input: Input,

    #[command(flatten)]
    pub output: Output,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Input {
    /// Path of an Obsidian LaTeX suite snippet file to convert **from**.
    /// This is _not_ directly `data.json` in the vault. (though it could be in the future. hm.)
    #[arg(long)]
    pub ols_in: Option<PathBuf>,

    /// Path of an UltiSnips snippet file to convert **from**. Only one at a time.
    /// program repeatedly for more.
    // TODO: actually allow more than only n=1 inputs and concat them -- might conflict with the
    // stateful priority mechanism of the UltiSnips serializer
    #[arg(long, required_unless_present = "ols_in")]
    pub ultisnips_in: Option<PathBuf>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
pub struct Output {
    /// Path of an Obsidian LaTeX suite snippet file to write **to**. This is again _not_
    /// directly the `data.json` of OLS in the vault.
    #[arg(long, required_unless_present = "ultisnips_out")]
    pub ols_out: Option<PathBuf>,

    /// Path of an UltiSnips snippet file to write **to**.
    #[arg(long, required_unless_present = "ols_out")]
    pub ultisnips_out: Option<PathBuf>,
}
