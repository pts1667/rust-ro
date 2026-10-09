use std::path::Path;
use std::process::Command;

/// Each module is a separate cargo workspace: its manifest, and the wasm it installs as `config/wasm/<name>.wasm`.
const MODULES: [(&str, &str); 5] = [
    ("scripts/Cargo.toml", "game_scripts"),
    ("scripts/towns/Cargo.toml", "towns"),
    ("scripts/misc/Cargo.toml", "misc"),
    ("scripts/jobs/Cargo.toml", "jobs"),
    ("scripts/quests/Cargo.toml", "quests"),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let destination = root.join("config/wasm");
    std::fs::create_dir_all(&destination).expect("Cannot create Wasm asset directory");
    for (manifest, name) in MODULES {
        let status = Command::new("cargo")
            .current_dir(root)
            .args([
                "build",
                "--manifest-path",
                manifest,
                "--target",
                "wasm32-unknown-unknown",
                "--release",
                "--locked",
            ])
            .status()
            .expect("Cannot start cargo");
        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }
        let built = root.join(manifest).parent().unwrap().join("target/wasm32-unknown-unknown/release").join(format!("{name}.wasm"));
        std::fs::copy(&built, destination.join(format!("{name}.wasm"))).unwrap_or_else(|error| panic!("Cannot install {name}: {error}"));
        println!("Built config/wasm/{name}.wasm");
    }
}
