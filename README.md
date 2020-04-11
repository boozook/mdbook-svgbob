# Svgbob plugin for mdbook

[![Crate current version badge](https://img.shields.io/crates/v/mdbook-svgbob.svg)](https://crates.io/crates/mdbook-svgbob)
![](https://github.com/fzzr-/mdbook-svgbob/workflows/Tests/badge.svg)
![](https://github.com/fzzr-/mdbook-svgbob/workflows/Audit/badge.svg)

[Svgbob][]-based preprocessor for [mdbook][] transform your ascii diagrams into a svg.


## Usage


### Prerequisites

- [Rust][] toolchain, the easiest and canonical way is to use [Rustup][]
- [mdbook][], just run `cargo install mdbook`


### Installation

Get latest release:

```bash
cargo install mdbook-svgbob
```

Or install from repo:

```bash
cargo install --git https://github.com/fzzr-/mdbook-svgbob.git
```


### Integration

Add the preprocessor into your book manifest:

```toml
# book.toml

[preprocessor.svgbob]
text_width = 8.0
text_height = 16.0
class = "bob"
font_family = "arial"
font_size = 14.0
stroke_width = 2.0
# there's using css-variables from theme:
stroke_color = "var(--fg)" # see default theme / variables.css
background_color = "transparent" # also useful `var(--bg)`
# all properties are optional.
```

Then add code-block to some chapter:

````md
<!-- chapter_1.md -->

```bob
                                   .------------>-----------------.
       ┌-------------┐  .-.   .-.  |   ┌------┐  .-.   ┌-----┐    |    .-.   ┌------┐
  O-╮--| struct_name |-( : )-( | )-╰-╮-| name |-( : )--| tpe |--╮-╯---( | )--| body |--╭---O
    |  └-------------┘  `-'   `-'    | └------┘  `-'   └-----┘  |      `-'   └------┘  |
    |                                |                    .-.   |                      |
    |                                `------------<------( , )--'                      |
    |                                                     `-'                          |
    `----------------------------------------------------------------------------------'
```
````

That's all. ♥️

For more information about mdbook see [mdbook manual][mdbook.manual],
[svgbob spec][svgbob.spec] and [editor][svgbob.editor].


## Configuration

Contributions are highly appreciated and encouraged!
Don't hesitate to participate to discussions in the issues, propose new features and ask for help.

Useful hint for one-command builds crate + book:

1. add `command` to the manifest:

```toml
# book.toml

[preprocessor.svgbob]
command = "cargo run --manifest-path /path/to/mdbook-svgbob/Cargo.toml --quiet"
```

2. So then you only need to rebuild the book. Run something like this:

```bash
RUST_LOG=mdbook_svgbob=trace mdbook build
```


[Rust]: https://www.rust-lang.org
[Rustup]: https://rustup.rs

[mdbook]: https://crates.io/crates/mdbook
[mdbook.manual]: https://rust-lang.github.io/mdBook/

[Svgbob]: https://crates.io/crates/svgbob
[svgbob.spec]: https://ivanceras.github.io/#md/Svgbob/Specification.md
[svgbob.editor]: https://ivanceras.github.io/svgbob-editor/
