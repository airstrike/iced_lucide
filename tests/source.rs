//! Exercise the public generator against local source files and existing outputs.
use std::{fs, path::Path};

use iced_lucide::{Error, FONT_BYTES, build};
use tempfile::tempdir;

fn definition(dir: &Path, ttf: &Path, index: &Path) -> std::path::PathBuf {
    let config = dir.join("icons.toml");
    let module = dir.join("icon.rs");
    fs::write(&config, format!(
        "module = 'icon'\nmodule_target = '{}'\n[source]\nttf = '{}'\nindex = '{}'\n[icons]\nchat = 'conversation'\n",
        module.display(), ttf.display(), index.display(),
    )).unwrap();
    config
}

#[test]
fn either_index_format_selects_the_glyph_and_refreshes_cached_outputs() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    let font = dir.join("source.ttf");
    let index = dir.join("names");
    let subset = dir.join("lucide.ttf");
    let module = dir.join("icon.rs");
    let codes = iced_lucide::icons();
    let code = codes
        .iter()
        .find(|(n, _)| n == "message-circle-dashed")
        .unwrap()
        .1;
    fs::write(&font, FONT_BYTES).unwrap();
    fs::write(&index, format!(r#"{{"conversation":{code}}}"#)).unwrap();
    let config = definition(dir, &font, &index);
    build(&config).unwrap();

    let original = fs::read_to_string(&module).unwrap();
    assert!(original.contains(&format!("(\"chat\", \"\\u{{{code:X}}}\")")));
    let first_subset = fs::read(&subset).unwrap();
    let face = ttf_parser::Face::parse(&first_subset, 0).unwrap();
    assert!(face.glyph_index(char::from_u32(code).unwrap()).is_some());
    assert!(first_subset.len() < FONT_BYTES.len());

    fs::write(&module, original.replace("fn chat", "fn stale")).unwrap();
    build(&config).unwrap();
    assert_eq!(original, fs::read_to_string(&module).unwrap());

    // The release index bytes change while the effective mapping stays identical.
    fs::write(
        &index,
        format!(r#"<li><h4>conversation</h4><span class="unicode">&amp;#{code};</span></li>"#),
    )
    .unwrap();
    build(&config).unwrap();
    let after_index = fs::read_to_string(&module).unwrap();
    assert_ne!(original, after_index);
    assert_eq!(first_subset, fs::read(&subset).unwrap());

    // A valid different source font still covers the exact same requested icon.
    fs::write(&font, &first_subset).unwrap();
    build(&config).unwrap();
    assert_ne!(after_index, fs::read_to_string(&module).unwrap());
    let bytes = fs::read(&subset).unwrap();
    assert!(
        ttf_parser::Face::parse(&bytes, 0)
            .unwrap()
            .glyph_index(char::from_u32(code).unwrap())
            .is_some()
    );
}

#[test]
fn invalid_pairs_leave_sources_and_previous_outputs_intact() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path();
    let source = dir.join("source.ttf");
    let index = dir.join("names.json");
    fs::write(&source, FONT_BYTES).unwrap();
    let config = definition(dir, &source, &index);
    let subset = dir.join("lucide.ttf");
    fs::write(&subset, b"previous subset").unwrap();

    fs::write(&index, "{}").unwrap();
    assert!(
        matches!(build(&config), Err(Error::UnknownIcon { name, .. }) if name == "conversation")
    );
    fs::write(&index, r#"{"conversation":1114111}"#).unwrap();
    assert!(matches!(
        build(&config),
        Err(Error::MissingGlyph { code: 0x10ffff, .. })
    ));
    assert_eq!(fs::read(&subset).unwrap(), b"previous subset");

    let config = definition(dir, &subset, &index);
    assert!(matches!(
        build(&config),
        Err(Error::OutputOverwritesSource(_))
    ));
    assert_eq!(fs::read(&subset).unwrap(), b"previous subset");
}

#[test]
fn explicit_source_requires_both_paths() {
    let tmp = tempdir().unwrap();
    let config = tmp.path().join("icons.toml");
    fs::write(
        &config,
        "module='icon'\n[source]\nttf='only.ttf'\n[icons]\nchat='x'",
    )
    .unwrap();
    assert!(
        matches!(build(&config), Err(Error::Definition { detail, .. }) if detail.contains("index"))
    );
}
