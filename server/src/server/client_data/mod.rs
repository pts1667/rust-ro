//! Read-only HTTP server that publishes the extracted client data under `/data`.
//!
//! The tree is built by `tools/client-data-setup` from an installed kRO client: every resource the
//! client can ask for is extracted from the GRF archives, maps are narrowed to the ones the server
//! serves, and the loose `BGM` folder is copied along. Paths are the archive names the client uses,
//! lowercased, with the leading `data/` stripped (`sprite/…`, `texture/…`, `prontera.rsw`, `bgm/…`),
//! which is the same layout the client's `data_dir` option expects. The setup stores compressible
//! files only as `<name>.zst`, which `ServeDir` sends as-is with `Content-Encoding: zstd` when the
//! request accepts it; already-compressed formats stay identity so range requests keep working.
//!
//! Enabled plugins add the files of their `ui/` folder as `plugin/<name>/...`, listed in `/data/check` like the rest, so
//! clients fetch and cache them the same way (`docs/plugin-ui.md`). When plugins add items, skills or jobs, their catalog
//! is `plugin-catalog.json`, listed and cached the same way (`plugin_ui::catalog`).
//!
//! Music copied into `bgm/` after the setup ran is listed too: clients only fetch what `/data/check` names.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
use tracing::{error, info, warn};

use crate::server::Server;

/// Written by the setup tool next to `manifest.json`, one JSON object per file.
const CHECK_FILE: &str = "check.jsonl";
const BGM_FOLDER: &str = "bgm";

/// How often the loop checks whether the game server stopped.
const SHUTDOWN_POLL: Duration = Duration::from_millis(500);
const API_THREADS: usize = 1;

/// Serves the configured root until the server stops. Runs on its own runtime, so a slow client can
/// never starve the game.
pub fn serve(server: Arc<Server>) {
    let config = &server.configuration.client_data;
    if !std::path::Path::new(&config.root).is_dir() {
        error!(
            "The client data endpoint is disabled, its root {} is not a directory: run `cargo run --package tools --bin \
             client-data-setup --release` first",
            config.root
        );
        return;
    }
    if !std::path::Path::new(&config.root).join(CHECK_FILE).is_file() {
        warn!("{CHECK_FILE} is missing from {}, /data/check answers 404 until client-data-setup is run again", config.root);
    }
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(API_THREADS)
        .thread_name("client_data")
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            error!("The client data endpoint is disabled, its runtime failed to start: {error}");
            return;
        }
    };
    runtime.block_on(async {
        let listener = match tokio::net::TcpListener::bind((config.host.as_str(), config.port)).await {
            Ok(listener) => listener,
            Err(error) => {
                error!("The client data endpoint can not listen on {}:{}: {error}", config.host, config.port);
                return;
            }
        };
        info!("Client data is served on http://{}:{}/data from {}", config.host, config.port, config.root);
        let assets: Vec<(String, PathBuf)> =
            server.plugins().iter().map(|plugin| (plugin.name().to_string(), plugin.folder.join("ui"))).filter(|(_, folder)| folder.is_dir()).collect();
        let catalog = server.plugins().published_catalog().map(|catalog| serde_json::to_vec(catalog).expect("the plugin catalog is JSON"));
        let app = router(&config.root, &assets, catalog);
        let stopped = {
            let server = server.clone();
            async move {
                while server.is_alive() {
                    tokio::time::sleep(SHUTDOWN_POLL).await;
                }
            }
        };
        // No graceful shutdown: an open download would keep the server from stopping
        tokio::select! {
            result = axum::serve(listener, app) => {
                if let Err(error) = result {
                    error!("The client data endpoint stopped: {error}");
                }
            }
            _ = stopped => {}
        }
    });
    runtime.shutdown_background();
}

/// `/data/<path>` serves `<root>/<path>`. `ServeDir` handles percent-decoded UTF-8 names (client
/// paths contain Korean folders), range requests for audio, `Last-Modified` and refuses to leave the
/// root. A compressible file exists only as `<path>.zst` and is sent with `Content-Encoding: zstd`
/// when the request accepts it; files stored as-is stay available to every client.
///
/// `plugins` are the `ui/` folders of the enabled plugins by plugin name, served under `/data/plugin/<name>/`, and
/// `catalog` the plugin catalog's JSON, served as `/data/plugin-catalog.json`. With any, or with `bgm/` files the setup's
/// list lacks or lists with another size, `/data/check` is built once here: the setup's list with the music on disk,
/// followed by the plugin files.
pub fn router(root: &str, plugins: &[(String, PathBuf)], catalog: Option<Vec<u8>>) -> Router {
    let mut router = Router::new();
    let bgm = bgm_files(Path::new(root));
    let setup = setup_list(Path::new(root), !plugins.is_empty() || !bgm.is_empty() || catalog.is_some());
    let (mut setup, bgm_changed) = with_bgm(setup, &bgm);
    if catalog.is_none() && plugins.is_empty() && !bgm_changed {
        let check = ServeFile::new_with_mime(
            Path::new(root).join(CHECK_FILE),
            &"application/x-ndjson".parse::<mime::Mime>().expect("valid mime type"),
        )
        .precompressed_zstd();
        router = router.route_service("/data/check", check);
    } else {
        if let Some(catalog) = catalog {
            setup.extend_from_slice(&check_entry(plugin_ui::catalog::CATALOG_PATH, &catalog));
            let catalog = Arc::new(catalog);
            router = router.route(
                &format!("/data/{}", plugin_ui::catalog::CATALOG_PATH),
                axum::routing::get(move || {
                    let catalog = catalog.clone();
                    async move { ([(axum::http::header::CONTENT_TYPE, "application/json")], catalog.as_ref().clone()) }
                }),
            );
        }
        let list = Arc::new(check_list(setup, plugins));
        router = router.route(
            "/data/check",
            axum::routing::get(move || {
                let list = list.clone();
                async move { ([(axum::http::header::CONTENT_TYPE, "application/x-ndjson")], list.as_ref().clone()) }
            }),
        );
        for (name, folder) in plugins {
            router = router.nest_service(&format!("/data/plugin/{name}"), ServeDir::new(folder).append_index_html_on_directories(false));
        }
    }
    router.nest_service(
        "/data",
        ServeDir::new(root)
            .append_index_html_on_directories(false)
            .precompressed_zstd(),
    )
}

/// The setup's check list, decompressed when it was only stored as `.zst`, rebuilt from `manifest.json` when an older setup
/// wrote none. Only read when `needed`: otherwise the file is served as it is.
fn setup_list(root: &Path, needed: bool) -> Vec<u8> {
    if !needed {
        return Vec::new();
    }
    let mut list = std::fs::read(root.join(CHECK_FILE))
        .or_else(|_| std::fs::read(root.join(format!("{CHECK_FILE}.zst"))).and_then(|compressed| zstd::stream::decode_all(compressed.as_slice())))
        .unwrap_or_else(|_| manifest_lines(root));
    if !list.is_empty() && !list.ends_with(b"\n") {
        list.push(b'\n');
    }
    list
}

/// The setup's list followed by a line per plugin file. A client takes this list as everything the server has, so it must
/// keep the setup's files.
fn check_list(mut list: Vec<u8>, plugins: &[(String, PathBuf)]) -> Vec<u8> {
    for (name, folder) in plugins {
        let mut files = Vec::new();
        collect_files(folder, folder, &mut files);
        files.sort();
        for (relative, file) in files {
            match check_line(&format!("plugin/{name}/{relative}"), &file) {
                Ok(line) => list.extend_from_slice(&line),
                Err(error) => warn!("Plugin {name} asset {} is not served: {error}", file.display()),
            }
        }
    }
    list
}

/// `{ path, size, md5 }` and a newline, the line format the setup writes.
fn check_line(path: &str, file: &Path) -> std::io::Result<Vec<u8>> {
    Ok(check_entry(path, &std::fs::read(file)?))
}

fn check_entry(path: &str, bytes: &[u8]) -> Vec<u8> {
    let mut line = serde_json::json!({ "path": path, "size": bytes.len(), "md5": format!("{:x}", md5::compute(bytes)) }).to_string().into_bytes();
    line.push(b'\n');
    line
}

/// The music under `bgm/`: listed path (`bgm/03.mp3`), file and size on disk. `.zst` files are left out, the setup lists
/// them under their plain name.
fn bgm_files(root: &Path) -> Vec<(String, PathBuf, u64)> {
    let folder = root.join(BGM_FOLDER);
    let mut files = Vec::new();
    collect_files(&folder, &folder, &mut files);
    files.sort();
    files
        .into_iter()
        .filter(|(relative, _)| !relative.ends_with(".zst"))
        .filter_map(|(relative, file)| {
            let size = file.metadata().ok()?.len();
            Some((format!("{BGM_FOLDER}/{relative}"), file, size))
        })
        .collect()
}

/// `list` with the line of every music file listed with another size replaced by the file on disk (what `ServeDir` sends),
/// and a line added for every one it lacks; files listed with their size are not read. Also says whether anything changed.
fn with_bgm(list: Vec<u8>, bgm: &[(String, PathBuf, u64)]) -> (Vec<u8>, bool) {
    if bgm.is_empty() {
        return (list, false);
    }
    let on_disk: HashMap<&str, (&Path, u64)> = bgm.iter().map(|(path, file, size)| (path.as_str(), (file.as_path(), *size))).collect();
    let mut listed = HashSet::new();
    let mut changed = false;
    let mut merged = Vec::with_capacity(list.len());
    for line in list.split_inclusive(|byte| *byte == b'\n') {
        let entry: Option<serde_json::Value> = serde_json::from_slice(line.trim_ascii()).ok();
        let found = entry.as_ref().and_then(|entry| entry["path"].as_str()).and_then(|path| on_disk.get_key_value(path));
        let Some((path, (file, size))) = found else {
            merged.extend_from_slice(line);
            continue;
        };
        listed.insert(*path);
        if entry.as_ref().and_then(|entry| entry["size"].as_u64()) == Some(*size) {
            merged.extend_from_slice(line);
            continue;
        }
        match check_line(path, file) {
            Ok(fresh) => {
                merged.extend_from_slice(&fresh);
                changed = true;
            }
            Err(error) => {
                warn!("Music {} keeps its listed entry, it can not be read: {error}", file.display());
                merged.extend_from_slice(line);
            }
        }
    }
    if !merged.is_empty() && !merged.ends_with(b"\n") {
        merged.push(b'\n');
    }
    for (path, file, _) in bgm.iter().filter(|(path, _, _)| !listed.contains(path.as_str())) {
        match check_line(path, file) {
            Ok(line) => {
                merged.extend_from_slice(&line);
                changed = true;
            }
            Err(error) => warn!("Music {} is not listed, it can not be read: {error}", file.display()),
        }
    }
    (merged, changed)
}

fn manifest_lines(root: &Path) -> Vec<u8> {
    #[derive(serde::Deserialize)]
    struct Manifest {
        files: Vec<serde_json::Value>,
    }
    let manifest = std::fs::read(root.join("manifest.json")).ok().and_then(|bytes| serde_json::from_slice::<Manifest>(&bytes).ok());
    let Some(manifest) = manifest else {
        warn!("{CHECK_FILE} and manifest.json are missing from {}, /data/check lists only plugin and music files", root.display());
        return Vec::new();
    };
    manifest.files.iter().flat_map(|file| format!("{file}\n").into_bytes()).collect()
}

fn collect_files(base: &Path, folder: &Path, files: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = std::fs::read_dir(folder) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(base, &path, files);
        } else if let Ok(relative) = path.strip_prefix(base) {
            let relative = relative.components().map(|part| part.as_os_str().to_string_lossy().into_owned()).collect::<Vec<_>>().join("/");
            files.push((relative, path));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    fn test_root(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("rust_ro_client_data_router_test_{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("sprite")).unwrap();
        std::fs::write(dir.join("manifest.json"), b"{\"files\":[]}").unwrap();
        std::fs::write(dir.join("sprite/poring.spr"), b"SPR").unwrap();
        dir
    }

    fn get(uri: &str) -> Request<Body> {
        Request::builder().uri(uri).body(Body::empty()).unwrap()
    }

    #[tokio::test]
    async fn serves_files_under_data_and_refuses_escapes() {
        let root = test_root("files");
        let app = router(root.to_str().unwrap(), &[], None);

        let response = app.clone().oneshot(get("/data/sprite/poring.spr")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"SPR");

        let response = app.clone().oneshot(get("/data/manifest.json")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let response = app.oneshot(get("/data/../Cargo.toml")).await.unwrap();
        assert_ne!(response.status(), StatusCode::OK, "paths outside the root must not be served");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn serves_check_list_as_jsonl_and_prefers_its_zstd_sibling() {
        let root = test_root("check");
        let app = router(root.to_str().unwrap(), &[], None);

        let response = app.clone().oneshot(get("/data/check")).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "no check list written yet");

        let jsonl = b"{\"path\":\"sprite/poring.spr\",\"size\":3,\"md5\":\"abc\"}\n";
        std::fs::write(root.join(CHECK_FILE), jsonl).unwrap();
        let response = app.clone().oneshot(get("/data/check")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("content-type").unwrap(), "application/x-ndjson");
        assert!(response.headers().get("content-encoding").is_none());
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], jsonl);

        std::fs::write(root.join(format!("{CHECK_FILE}.zst")), zstd::bulk::compress(jsonl, 3).unwrap()).unwrap();
        let request = Request::builder()
            .uri("/data/check")
            .header("accept-encoding", "zstd")
            .body(Body::empty())
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("content-encoding").unwrap(), "zstd");
        assert_eq!(response.headers().get("content-type").unwrap(), "application/x-ndjson");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(zstd::bulk::decompress(&body, 1024).unwrap(), jsonl);

        let response = app.oneshot(get("/data/sprite/poring.spr")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK, "the check route must not shadow other files");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn serves_compressed_only_files_and_falls_back_to_identity() {
        let root = test_root("zstd");
        // Compressible files exist only as `.zst`: `poring.spr` from the fixture is stored as-is, and
        // `only-zstd.spr` has no identity file.
        let compressed = zstd::bulk::compress(b"SPR", 3).unwrap();
        std::fs::write(root.join("sprite/only-zstd.spr.zst"), &compressed).unwrap();
        let app = router(root.to_str().unwrap(), &[], None);

        let request = Request::builder()
            .uri("/data/sprite/only-zstd.spr")
            .header("accept-encoding", "zstd")
            .body(Body::empty())
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("content-encoding").unwrap(), "zstd");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        // The endpoint sends the compressed file untouched; the HTTP client decodes it.
        assert_eq!(zstd::bulk::decompress(&body, 64).unwrap(), b"SPR");

        // Without zstd the compressed-only file cannot be served.
        let response = app.clone().oneshot(get("/data/sprite/only-zstd.spr")).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // Stored files are served to every client; a zstd request falls back to them.
        for uri in ["/data/sprite/poring.spr", "/data/manifest.json"] {
            let request = Request::builder()
                .uri(uri)
                .header("accept-encoding", "zstd")
                .body(Body::empty())
                .unwrap();
            let response = app.clone().oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert!(response.headers().get("content-encoding").is_none());
        }

        let response = app.oneshot(get("/data/sprite/poring.spr")).await.unwrap();
        assert!(response.headers().get("content-encoding").is_none());
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"SPR");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn plugin_assets_are_served_and_listed_after_the_setup_files() {
        let root = test_root("plugins");
        std::fs::write(root.join(CHECK_FILE), b"{\"path\":\"sprite/poring.spr\",\"size\":3,\"md5\":\"abc\"}").unwrap();
        let folder = root.join("ui_shop_ui");
        std::fs::create_dir_all(folder.join("frames")).unwrap();
        std::fs::write(folder.join("coin.png"), b"PNG").unwrap();
        std::fs::write(folder.join("frames/a.png"), b"A").unwrap();
        let app = router(root.to_str().unwrap(), &[("ui_shop".into(), folder)], None);

        let response = app.clone().oneshot(get("/data/check")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let lines: Vec<serde_json::Value> =
            body.split(|b| *b == b'\n').filter(|line| !line.is_empty()).map(|line| serde_json::from_slice(line).unwrap()).collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0]["path"], "sprite/poring.spr");
        assert_eq!(lines[1]["path"], "plugin/ui_shop/coin.png");
        assert_eq!(lines[1]["md5"], format!("{:x}", md5::compute(b"PNG")));
        assert_eq!(lines[2]["path"], "plugin/ui_shop/frames/a.png");

        let response = app.clone().oneshot(get("/data/plugin/ui_shop/frames/a.png")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(&to_bytes(response.into_body(), usize::MAX).await.unwrap()[..], b"A");
        let response = app.clone().oneshot(get("/data/sprite/poring.spr")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK, "setup files are still served");
        let response = app.oneshot(get("/data/plugin/ui_shop/../../check.jsonl")).await.unwrap();
        assert_ne!(response.status(), StatusCode::OK);

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn the_plugin_catalog_is_served_and_listed_before_the_plugin_assets() {
        let root = test_root("catalog");
        std::fs::write(root.join(CHECK_FILE), b"{\"path\":\"sprite/poring.spr\",\"size\":3,\"md5\":\"abc\"}").unwrap();
        let folder = root.join("lucky_ui");
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join("ticket.png"), b"PNG").unwrap();
        let catalog = b"{\"items\":[],\"skills\":[],\"jobs\":[]}".to_vec();
        let app = router(root.to_str().unwrap(), &[("lucky".into(), folder)], Some(catalog.clone()));

        let response = app.clone().oneshot(get("/data/check")).await.unwrap();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let lines: Vec<serde_json::Value> =
            body.split(|b| *b == b'\n').filter(|line| !line.is_empty()).map(|line| serde_json::from_slice(line).unwrap()).collect();
        let paths: Vec<_> = lines.iter().map(|line| line["path"].as_str().unwrap()).collect();
        assert_eq!(paths, ["sprite/poring.spr", plugin_ui::catalog::CATALOG_PATH, "plugin/lucky/ticket.png"]);
        assert_eq!(lines[1]["md5"], format!("{:x}", md5::compute(&catalog)));
        assert_eq!(lines[1]["size"], catalog.len());

        let response = app.clone().oneshot(get(&format!("/data/{}", plugin_ui::catalog::CATALOG_PATH))).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
        assert_eq!(to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec(), catalog);
        let response = app.oneshot(get("/data/plugin/lucky/ticket.png")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn music_copied_after_the_setup_is_listed_as_it_is_on_disk() {
        let root = test_root("bgm");
        std::fs::create_dir_all(root.join("bgm")).unwrap();
        std::fs::write(root.join("bgm/08.mp3"), b"LONGER MP3").unwrap();
        std::fs::write(root.join("bgm/149.mp3"), b"MP3").unwrap();
        std::fs::write(root.join("bgm/03.mp3"), b"ID3").unwrap();
        let setup = concat!(
            "{\"path\":\"sprite/poring.spr\",\"size\":3,\"md5\":\"abc\"}\n",
            "{\"path\":\"bgm/08.mp3\",\"size\":3,\"md5\":\"stale\"}\n",
            "{\"path\":\"bgm/149.mp3\",\"size\":3,\"md5\":\"kept\"}",
        );
        std::fs::write(root.join(CHECK_FILE), setup).unwrap();
        let app = router(root.to_str().unwrap(), &[], None);

        let response = app.clone().oneshot(get("/data/check")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("content-type").unwrap(), "application/x-ndjson");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let lines: Vec<serde_json::Value> =
            body.split(|b| *b == b'\n').filter(|line| !line.is_empty()).map(|line| serde_json::from_slice(line).unwrap()).collect();
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0]["path"], "sprite/poring.spr");
        assert_eq!(lines[1]["path"], "bgm/08.mp3");
        assert_eq!(lines[1]["size"], 10);
        assert_eq!(lines[1]["md5"], format!("{:x}", md5::compute(b"LONGER MP3")));
        assert_eq!(lines[2]["md5"], "kept", "a file listed with its size is not hashed again");
        assert_eq!(lines[3]["path"], "bgm/03.mp3");
        assert_eq!(lines[3]["md5"], format!("{:x}", md5::compute(b"ID3")));

        std::fs::remove_file(root.join("bgm/03.mp3")).unwrap();
        std::fs::write(root.join("bgm/08.mp3"), b"MP3").unwrap();
        let app = router(root.to_str().unwrap(), &[], None);
        let response = app.oneshot(get("/data/check")).await.unwrap();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], setup.as_bytes(), "a list that names the music on disk is served as the setup wrote it");

        let _ = std::fs::remove_dir_all(&root);
    }
}
