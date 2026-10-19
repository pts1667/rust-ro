use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Builds the `scripts` workspace and installs every module as `config/wasm/modules/<name>.wasm`.
/// A module is a crate under `scripts/` that builds a `cdylib`; its directory name is its module name.
/// Crates under `scripts/plugins/` are plugins instead, installed as `config/plugins/<name>/plugin.wasm` next to their manifests.
fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let scripts = root.join("scripts");
    let destination = root.join("config/wasm/modules");
    std::fs::create_dir_all(&destination).expect("Cannot create Wasm module directory");
    check_plugin_manifests(&scripts);

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
    let mut plugins = 0;
    for directory in module_directories(&scripts) {
        let name = directory.file_name().unwrap().to_string_lossy().into_owned();
        if directory.parent().is_some_and(|parent| parent.ends_with("plugins")) {
            let folder = root.join("config/plugins").join(&name);
            std::fs::create_dir_all(&folder).expect("Cannot create the plugin directory");
            std::fs::copy(built.join(format!("{name}.wasm")), folder.join("plugin.wasm"))
                .unwrap_or_else(|error| panic!("Cannot install plugin {name}: {error}"));
            plugins += 1;
            continue;
        }
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
    println!("Built {} modules into config/wasm/modules and {plugins} plugins into config/plugins", installed.len());
}

/// `plugin_module!` compiles each plugin's `info.json` into it, and a module with an invalid one is refused at boot with
/// little detail, so the build checks them first.
fn check_plugin_manifests(scripts: &Path) {
    let mut invalid = false;
    for directory in module_directories(scripts).into_iter().filter(|directory| directory.parent().is_some_and(|parent| parent.ends_with("plugins"))) {
        let path = directory.join("info.json");
        let checked = std::fs::read_to_string(&path).map_err(|error| format!("cannot read it: {error}")).and_then(|text| script_sdk::PluginManifest::parse(&text));
        if let Err(error) = checked {
            eprintln!("{}: {error}", path.display());
            invalid = true;
        }
    }
    if invalid {
        std::process::exit(1);
    }
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
