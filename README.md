# Svgbob plugin for mdbook

[Svgbob][]-based preprocessor for [mdbook][] transform your ascii diagrams into a svg.

This renders a code-block marked `bob` into neat svg diagrams and inline it into the output.


## Usage

Firstly add the following to your book's manifest file
(usually `book.toml`)

```toml
[preprocessor.svgbob] # all fields by default
```
See [config](#config) for more information.


Add some ascii graphics:

_chapter_1.md_
````md
```bob
    0       3
     *-------*      +y
  1 /|    2 /|       ^
   *-------* |       |
   | |4    | |7      | ◄╮
   | *-----|-*     ⤹ +-----> +x
   |/      |/       / ⤴
   *-------*       v
  5       6      +z
```
````

Build your book and open:
```
mdbook build
open book/index.html
```

### Config

Default preprocessor config:

```toml
[preprocessor.svgbob]
# svgbob configuration:
# doc: https://docs.rs/svgbob/latest/svgbob/struct.Settings.html
# default values: https://github.com/ivanceras/svgbob/blob/master/packages/svgbob/src/settings.rs#L29-L38
font_size = 14
font_family = "Iosevka Fixed, monospace"
fill_color = "black"
background = "transparent"               # default overridden, differs from svgbob's default
stroke_color = "var(--fg)"               # default overridden, differs from svgbob's default
stroke_width = 2.0
scale = 8.0
include_backdrop = true
include_styles = true
include_defs = true

# preprocessor configuration:
code_block = "bob"                       # render only code-blocks marked as "bob", e.g.: ```bob
```

All properties are optional.

See [svgbob's settings doc](https://docs.rs/svgbob/latest/svgbob/struct.Settings.html) and [default values](https://github.com/ivanceras/svgbob/blob/master/packages/svgbob/src/settings.rs#L29-L38]).


- - -


For more information about mdbook see [mdbook manual][mdbook.manual],
[svgbob spec][svgbob.spec] and [editor][svgbob.editor].


## Contribution

Contributions are appreciated.
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
