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
        tracing::debug!("Creating crate directory at {:?}", lib_path_buf);
        std::fs::create_dir_all(&lib_path_buf)?;
    }
    tree.as_deref_mut()
        .map(|t| t.begin_child(name.as_ref().to_string()));

    if !dry {
        tracing::debug!(
            "Creating crate Cargo.toml file as {:?}",
            cargo_toml_path_buf
        );
        let mut cargo_toml = std::fs::File::create(&cargo_toml_path_buf)?;
        cargo_toml.write_all(include_bytes!("../templates/lib/Cargo.toml"))?;
    }
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("Cargo.toml".to_string()));

    if !dry {
        tracing::debug!("Creating crate src directory at {:?}", src_path_buf);
        std::fs::create_dir_all(&src_path_buf)?;
    }
    tree.as_deref_mut()
        .map(|t| t.begin_child("src".to_string()));

    if !dry {
        tracing::debug!("Creating lib.rs file as {:?}", lib_rs_path_buf);
        let mut lib_rs = std::fs::File::create(&lib_rs_path_buf)?;
        lib_rs.write_all(include_bytes!("../templates/lib/lib.rs"))?;
    }
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("main.rs".to_string()));

    tree.as_deref_mut().map(|t| t.end_child()); // <- src/
    tree.map(|t| t.end_child()); // <- <name>/

    Ok(())
}
