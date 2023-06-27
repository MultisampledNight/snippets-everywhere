they told me i could become anything so i became an IR for snippet files

## Why

Imagine you're using both the [OLS] (Obsidian LaTeX suite) in [Obsidian] and [UltiSnips] in [NeoVim] quite often. Sometimes, when typing longer LaTeX documents in [Neovim], you notice that it'd be insanely neat to have the exact same snippets you're used to from taking notes in [Obsidian] anyway also in larger documents.

This program partly solves that by converting between the snippet files of [OLS] and [UltiSnips] at will.

## Installation

Assumed you have Rust already installed (if not, see [The Book]) and you're not running an exotic platform, this is as simple as:

```
cargo install --git https://github.com/MultisampledNight/snippets-everywhere
```

## Usage

Give `--{ultisnips,ols}-in` an input path, and `--{ultisnips,ols}-out` a corresponding output path.

```
snippets-everywhere --ultisnips-in in-file.snippets --ols-out out-file.json
snippets-everywhere --ols-in in-file.json --ultisnips-out out-file.snippets
```

Do note that `--ols-in` expects the input JSON file to be the one you _see_ in the settings of [OLS]. Which you can get partly programmatically using this very sane construct...

```shell
python -c 'import json, pathlib; print(json.loads(pathlib.Path("$YOUR_VAULT_PATH/.obsidian/plugins/obsidian-latex-suite/data.json").expanduser().read_text()["snippets"]))'
```

...and replacing `$YOUR_VAULT_PATH` with your real vault path.

## Caveats

- The [UltiSnips] snippet _parser_ as triggered through using `--ultisnips-in` tries to replicate the parsing of UltiSnips itself as closely as reasonably possible. This also includes the same surprising behaviors: `"wow"` as trigger is parsed as `"wow"`, unquoted, but `"wow more"` is parsed as `wow more`, quoted.
- Also, parsing and following `priority` and `extends` directives in the [UltiSnips] parser isn't implemented. Would be easy to add, though.
- Comments are not preserved, and not even parsed by the input backends, just skipped.
- The [OLS] output is very condensed, and not pretty printed. If you want or need pretty printing, you can throw it through `python -m json.tool`.

## FAQ

- An... IR?

  Yeah, I structured this a bit like I'd do in a compiler. It's more of a meme than anything serious, though.

- Can this take input and output on stdin and stdout instead of writing to files?

  On Unixalikes, you can just use `/dev/stdin` and `/dev/stdout`, respectively. I'm not sure about other platforms.

- Could other backends be added as well? How does the backend system even work?

  Open an issue describing where to find the format you want to parse and other details! I might implement it, I might not. Either way, if you know Rust (or want to learn it, check out [The Book]) see the module doccomment in [`src/backends/mod.rs`]!

[OLS]: https://github.com/artisticat1/obsidian-latex-suite
[Obsidian]: https://obsidian.md/
[UltiSnips]: https://github.com/SirVer/ultisnips
[NeoVim]: https://neovim.io

[The Book]: https://doc.rust-lang.org/stable/book/
[`src/backends/mod.rs`]: ./src/backends/mod.rs

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
