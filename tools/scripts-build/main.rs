use std::path::Path;
use std::process::Command;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let status = Command::new("cargo")
        .current_dir(root)
        .args([
            "build",
            "--manifest-path",
            "scripts/Cargo.toml",
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
    let destination = root.join("config/wasm");
    std::fs::create_dir_all(&destination).expect("Cannot create Wasm asset directory");
    std::fs::copy(
        root.join("scripts/target/wasm32-unknown-unknown/release/game_scripts.wasm"),
        destination.join("game_scripts.wasm"),
    )
    .expect("Cannot install compiled game scripts");
    println!("Built config/wasm/game_scripts.wasm");
}
