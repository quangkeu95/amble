use std::io::Write;
use std::path::Path;

use eyre::Result;
use ptree::TreeBuilder;
use tracing::instrument;

/// Creates a new lib crate.
#[instrument(name = "lib", skip(dir, name, dry, tree))]
pub(crate) fn create(
    dir: &Path,
    name: impl AsRef<str>,
    dry: bool,
    mut tree: Option<&mut TreeBuilder>,
) -> Result<()> {
    tracing::info!("Creating lib crate");

    let lib_path_buf = dir.join(name.as_ref());
    let src_path_buf = lib_path_buf.join("src");
    let cargo_toml_path_buf = lib_path_buf.join("Cargo.toml");
    let lib_rs_path_buf = lib_path_buf.join("src").join("lib.rs");

    if !dry {
        tracing::debug!("Creating {:?} directory", dir);
        std::fs::create_dir_all(dir)?;
    }
    tree.as_deref_mut()
        .map(|t| t.begin_child("crates".to_string()));

    if !dry {
        tracing::debug!("Creating {:?} directory", lib_path_buf);
        std::fs::create_dir_all(&lib_path_buf)?;
    }
    tree.as_deref_mut()
        .map(|t| t.begin_child(name.as_ref().to_string()));

    if !dry {
        tracing::debug!("Writing {:?}", cargo_toml_path_buf);
        fill_cargo(&cargo_toml_path_buf, name.as_ref())?;
    }
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("Cargo.toml".to_string()));

    if !dry {
        tracing::debug!("Creating {:?} directory", src_path_buf);
        std::fs::create_dir_all(&src_path_buf)?;
    }
    tree.as_deref_mut()
        .map(|t| t.begin_child("src".to_string()));

    if !dry {
        tracing::debug!("Writing {:?}", lib_rs_path_buf);
        let lib_contents = r#"#![doc = include_str!("../README.md")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

mod placeholder {
    struct Placeholder;
}
        "#;
        let mut lib_rs = std::fs::File::create(&lib_rs_path_buf)?;
        lib_rs.write_all(lib_contents.as_bytes())?;
    }
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("lib.rs".to_string()));

    tree.as_deref_mut().map(|t| t.end_child()); // <- src/
    tree.as_deref_mut().map(|t| t.end_child()); // <- <name>/
    tree.map(|t| t.end_child()); // <- crates

    Ok(())
}

/// Writes binary contents to the `Cargo.toml` file located at [file].
pub(crate) fn fill_cargo(file: &Path, name: &str) -> Result<()> {
    let mut manifest = toml_edit::Document::new();
    manifest["package"] = toml_edit::Item::Table(toml_edit::Table::new());
    manifest["package"]["name"] = toml_edit::value(name);
    manifest["package"]["description"] = toml_edit::value(format!("{} crate", name));
    let inline =
        toml_edit::Item::Value(toml_edit::Value::InlineTable(toml_edit::InlineTable::new()));
    manifest["package"]["version"] = inline.clone();
    manifest["package"]["version"]["workspace"] = toml_edit::value(true);
    manifest["package"]["edition"] = inline.clone();
    manifest["package"]["edition"]["workspace"] = toml_edit::value(true);
    manifest["package"]["authors"] = inline.clone();
    manifest["package"]["authors"]["workspace"] = toml_edit::value(true);
    manifest["package"]["license"] = inline.clone();
    manifest["package"]["license"]["workspace"] = toml_edit::value(true);
    manifest["package"]["repository"] = inline.clone();
    manifest["package"]["repository"]["workspace"] = toml_edit::value(true);
    manifest["package"]["homepage"] = inline.clone();
    manifest["package"]["homepage"]["workspace"] = toml_edit::value(true);

    manifest["dependencies"] = toml_edit::Item::Table(toml_edit::Table::new());
    manifest["dependencies"]["serde"] = inline.clone();
    manifest["dependencies"]["serde"]["workspace"] = toml_edit::value(true);
    manifest["dependencies"]["serde_json"] = inline.clone();
    manifest["dependencies"]["serde_json"]["workspace"] = toml_edit::value(true);
    manifest["dependencies"]["eyre"] = inline.clone();
    manifest["dependencies"]["eyre"]["workspace"] = toml_edit::value(true);
    manifest["dependencies"]["tracing"] = inline.clone();
    manifest["dependencies"]["tracing"]["workspace"] = toml_edit::value(true);

    let mut file = std::fs::File::create(file)?;
    file.write_all(manifest.to_string().as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use tempfile::tempdir;

    #[test]
    fn test_fill_cargo() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let proj_name = "common";
        let cargo_toml_path_buf = dir_path_buf.join("Cargo.toml");
        fill_cargo(&cargo_toml_path_buf, proj_name).unwrap();
        assert!(cargo_toml_path_buf.exists());

        // Validate the cargo.toml file contents
        let mut cargo_toml = File::open(cargo_toml_path_buf).unwrap();
        let mut cargo_toml_contents = String::new();
        cargo_toml.read_to_string(&mut cargo_toml_contents).unwrap();
        let expected_contents = r#"[package]
name = "common"
description = "common crate"
version = { workspace = true }
edition = { workspace = true }
authors = { workspace = true }
license = { workspace = true }
repository = { workspace = true }
homepage = { workspace = true }

[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
eyre = { workspace = true }
tracing = { workspace = true }
"#;
        assert_eq!(cargo_toml_contents, expected_contents);
    }

    #[test]
    fn test_create() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let crates_path_buf = dir_path_buf.join("crates");
        let project_name = "example";
        let project_path = crates_path_buf.join(project_name);
        create(&crates_path_buf, project_name, false, None).unwrap();

        assert!(project_path.exists());
        assert!(project_path.join("src").exists());
        assert!(project_path.join("src").join("lib.rs").exists());
        assert!(project_path.join("Cargo.toml").exists());

        let mut lib_rs = File::open(project_path.join("src").join("lib.rs")).unwrap();
        let mut lib_rs_contents = String::new();
        lib_rs.read_to_string(&mut lib_rs_contents).unwrap();
        assert!(lib_rs_contents.len() > 0);
    }

    #[test]
    fn test_create_dry_run() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let crates_path_buf = dir_path_buf.join("crates");
        let project_name = "example";
        let project_path = crates_path_buf.join(project_name);
        create(&crates_path_buf, project_name, true, None).unwrap();

        assert!(!project_path.exists());
        assert!(!project_path.join("src").exists());
        assert!(!project_path.join("src").join("lib.rs").exists());
        assert!(!project_path.join("Cargo.toml").exists());
    }
}