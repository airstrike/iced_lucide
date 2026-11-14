# iced_lucide

A compile-time, type-safe [Lucide](https://lucide.dev) icon font library for [`iced`](https://github.com/iced-rs/iced).

No network calls. The Lucide TTF and icon map are bundled directly into the library.

## Usage

Create a `.toml` file in your crate with the icons you need:

```toml
# fonts/my-icons.toml
module = "icon"

[icons]
edit   = "pencil"
save   = "save"
trash  = "trash-2"
search = "search"
```

Each key is the Rust function name; each value is the [Lucide icon name](https://lucide.dev/icons).

Add `iced_lucide` to your `build-dependencies`:

```toml
[build-dependencies]
iced_lucide = { git = "https://github.com/airstrike/iced_lucide" }
```

Call `build` in your `build.rs`:

```rust
pub fn main() {
    iced_lucide::build("fonts/my-icons.toml").expect("Build icon module");
}
```

This generates `src/icon.rs` and writes a subset font named `lucide.ttf` next to your TOML.

### Choose a Lucide release

The bundled font and name index come from [lucide-static 1.72.0](assets/README.md). To use another release, keep its matching TTF, name index and license together in your crate. Point the same build script at this definition:

```toml
# fonts/my-icons.toml
module = "icon"

[source]
ttf = "vendor/lucide-static-0.575.0/font/lucide.ttf"
index = "vendor/lucide-static-0.575.0/font/unicode.html"

[icons]
chat = "message-circle-dashed"
github = "github"
```

Paths in `[source]` are relative to the consuming crate root, like `module_target` and `ttf_target`. Keep input files separate from the generated subset. Use either the release’s `font/unicode.html` or its numeric `font/codepoints.json` when available. The HTML index lists drawable icons. Some JSON indices include removed names whose codepoints have no corresponding glyph. Requesting one produces a build error identifying the name and codepoint.

Lucide sometimes changes an outline, renames an icon, or removes a glyph such as `github`. Pinning the matching files restores the artwork and names for that release. Rust function names remain your choice under `[icons]`. Cargo tracks the definition and both source files. Changing outlines regenerates your subset even when the codepoints stay the same.

`build_all`, `icons()` and `FONT_BYTES` use the bundled release.

### Custom output paths

By default the generated module is written to `src/<module>.rs` and the
subsetted TTF lives next to the TOML. Both destinations can be overridden
with two optional fields, which is handy for Cargo examples, multi-file
layouts, or anywhere the generated module is not a member of `src/`:

```toml
# examples/playground/fonts/icons.toml
module        = "icon"
module_target = "examples/playground/icon.rs"
ttf_target    = "examples/playground/fonts/lucide.ttf"

[icons]
edit = "pencil"
save = "save"
```

Both paths are resolved relative to the crate root (the directory
containing `Cargo.toml`). `iced_lucide` creates parent directories as
needed and rewrites the `include_bytes!` call inside the generated module
so the relative distance between the two locations is correct.

Leaving either field out keeps the historical default, so existing
consumers don't need to change anything.

Register the font and use the generated functions:

```rust
mod icon;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .font(icon::FONT)
        .run()
}

fn view(&self) -> iced::Element<'_, ()> {
    iced::widget::row![icon::edit(), icon::save(), icon::trash()]
        .spacing(10)
        .into()
}
```

## All Icons (icon picker)

To generate every Lucide icon at once — useful for picker widgets or UI builders:

```rust
pub fn main() {
    iced_lucide::build_all("icon").expect("Build all icons");
}
```

The generated module exports `ALL_ICONS: &[(&str, &str)]`, a static list of
`(icon_name, codepoint_str)` pairs you can iterate at runtime:

```rust
use crate::icon::ALL_ICONS;

// in your view:
let buttons: Vec<_> = ALL_ICONS
    .iter()
    .map(|(name, cp)| button(text(*cp)).on_press(Msg::Pick(name.to_string())))
    .collect();
```

## Runtime enumeration

Add `iced_lucide` as a regular `[dependency]` (not just build) to call
`iced_lucide::icons()` at runtime:

```rust
for (name, codepoint) in iced_lucide::icons() {
    println!("{name}  U+{codepoint:04X}");
}
```

## Updating Lucide

For maintainers, the [bundled asset provenance](assets/README.md) records the pinned release and update procedure.

## Example

Run the [subset example](subset_example) or the [all-icons picker](all_icons_example):

```sh
cargo run -p subset_example
cargo run -p all_icons_example
```
