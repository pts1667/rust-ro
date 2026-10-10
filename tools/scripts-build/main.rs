use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Builds the `scripts` workspace and installs every module as `config/wasm/modules/<name>.wasm`.
/// A module is a crate under `scripts/` that builds a `cdylib`; its directory name is its module name.
fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let scripts = root.join("scripts");
    let destination = root.join("config/wasm/modules");
    std::fs::create_dir_all(&destination).expect("Cannot create Wasm module directory");

    let status = Command::new("cargo")
        .current_dir(root)
        .args(["build", "--manifest-path", "scripts/Cargo.toml", "--workspace", "--target", "wasm32-unknown-unknown", "--release", "--locked"])
        .status()
        .expect("Cannot start cargo");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    let built = scripts.join("target/wasm32-unknown-unknown/release");
    let mut installed = BTreeSet::new();
    for directory in module_directories(&scripts) {
        let name = directory.file_name().unwrap().to_string_lossy().into_owned();
        std::fs::copy(built.join(format!("{name}.wasm")), destination.join(format!("{name}.wasm")))
            .unwrap_or_else(|error| panic!("Cannot install {name}: {error}"));
        installed.insert(name);
    }
    for stale in std::fs::read_dir(&destination).unwrap().map(|entry| entry.unwrap().path()) {
        let name = stale.file_stem().unwrap().to_string_lossy();
        if matches!(stale.extension().and_then(|e| e.to_str()), Some("wasm" | "cwasm")) && !installed.contains(name.as_ref()) {
            std::fs::remove_file(&stale).unwrap_or_else(|error| panic!("Cannot remove {}: {error}", stale.display()));
        }
    }
    println!("Built {} modules into config/wasm/modules", installed.len());
}

/// Crates one or two levels below `scripts/` whose manifest declares a `cdylib`.
fn module_directories(scripts: &Path) -> Vec<PathBuf> {
    let children = |directory: &Path| -> Vec<PathBuf> {
        let mut paths: Vec<_> = std::fs::read_dir(directory).unwrap().map(|entry| entry.unwrap().path()).filter(|path| path.is_dir()).collect();
        paths.sort();
        paths
    };
    let mut found = Vec::new();
    for first in children(scripts) {
        if first.file_name().is_some_and(|name| name == "target") {
            continue;
        }
        for candidate in std::iter::once(first.clone()).chain(children(&first)) {
            let is_module = std::fs::read_to_string(candidate.join("Cargo.toml")).is_ok_and(|manifest| manifest.contains("cdylib"));
            if is_module {
                found.push(candidate);
            }
        }
    }
    found
}
