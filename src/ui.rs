use std::path::PathBuf;

use clap::{Args, Parser};

pub fn cmdline() -> Cmdline {
    Cmdline::parse()
}

#[derive(Parser, Debug)]
pub struct Cmdline {
    #[command(flatten)]
    input: Input,

    #[command(flatten)]
    output: Output,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Input {
    /// Path of an Obsidian LaTeX suite snippet file to convert **from**.
    /// This is _not_ directly `data.json` in the vault. (though it could be in the future. hm.)
    #[arg(long)]
    ols_in: Option<PathBuf>,

    /// Path of an UltiSnips snippet file to convert **from**. Only one at a time.
    /// program repeatedly for more.
    // TODO: actually allow more than only n=1 inputs and concat them
    #[arg(long, required_unless_present = "ols_in")]
    ultisnips_in: Option<PathBuf>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
pub struct Output {
    /// Path of an Obsidian LaTeX suite snippet file to write **to**. This is again _not_
    /// directly the `data.json` of OLS in the vault.
    #[arg(long, required_unless_present = "ultisnips_out")]
    ols_out: Option<PathBuf>,

    /// Path of an UltiSnips snippet file to write **to**.
    #[arg(long, required_unless_present = "ols_out")]
    ultisnips_out: Option<PathBuf>,
}
