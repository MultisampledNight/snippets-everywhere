they told me i could become anything so i became an IR for snippet files

## Why

Imagine you're using both the [OLS] (Obsidian LaTeX suite) in [Obsidian] and [UltiSnips] in [NeoVim] quite often. Sometimes, when typing longer LaTeX documents in [Neovim], you notice that it'd be insanely neat to have the exact same snippets you're used to from taking notes in [Obsidian] anyway also in larger documents.

This program partly solves that by converting between the snippet files of [OLS] and [UltiSnips] at will.

[OLS]: https://github.com/artisticat1/obsidian-latex-suite
[Obsidian]: https://obsidian.md/
[UltiSnips]: https://github.com/SirVer/ultisnips
[NeoVim]: https://neovim.io

## Installation

Assumed you have Rust already installed (if not, see [The Book]) and you're not running an exotic platform, this is as simple as:

```
cargo install --git https://github.com/MultisampledNight/snippets-everywhere
```

## Usage

Give `--{ultisnips,ols}-in` an input path, and `--{ultisnips,ols}-out` a corresponding output path.

```
snippets --ultisnips-in in-file.snippets --ols-out out-file.json
snippets --ols-in in-file.json --ultisnips-out out-file.snippets
```

Do note that `obsidian-to-ultisnips` expects the input JSON file to be the one you see in the settings of Obsidian LaTeX suite. Which you can get partly programmatically using this very sane construct...

```shell
python -c 'import json, pathlib; print(json.loads(pathlib.Path("$YOUR_VAULT_PATH/.obsidian/plugins/obsidian-latex-suite/data.json").expanduser().read_text()["snippets"]))'
```

...and replacing `$YOUR_VAULT_PATH` with your real vault path.


[The Book]: https://doc.rust-lang.org/stable/book/
