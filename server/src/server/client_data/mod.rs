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
//! clients fetch and cache them the same way (`docs/plugin-ui.md`).

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
use tracing::{error, info, warn};

use crate::server::Server;

/// Written by the setup tool next to `manifest.json`, one JSON object per file.
const CHECK_FILE: &str = "check.jsonl";

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
        let app = router(&config.root, &assets);
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
/// `plugins` are the `ui/` folders of the enabled plugins by plugin name, served under `/data/plugin/<name>/`. With any,
/// `/data/check` is built once here: the setup's list followed by the plugin files.
pub fn router(root: &str, plugins: &[(String, PathBuf)]) -> Router {
    let mut router = Router::new();
    if plugins.is_empty() {
        let check = ServeFile::new_with_mime(
            Path::new(root).join(CHECK_FILE),
            &"application/x-ndjson".parse::<mime::Mime>().expect("valid mime type"),
        )
        .precompressed_zstd();
        router = router.route_service("/data/check", check);
    } else {
        let list = Arc::new(check_list(Path::new(root), plugins));
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

/// The setup's check list (decompressed when it was only stored as `.zst`, rebuilt from `manifest.json` when an older setup
/// wrote none) with a line per plugin file. A client takes this list as everything the server has, so it must keep the
/// setup's files.
fn check_list(root: &Path, plugins: &[(String, PathBuf)]) -> Vec<u8> {
    let mut list = std::fs::read(root.join(CHECK_FILE))
        .or_else(|_| std::fs::read(root.join(format!("{CHECK_FILE}.zst"))).and_then(|compressed| zstd::stream::decode_all(compressed.as_slice())))
        .unwrap_or_else(|_| manifest_lines(root));
    if !list.is_empty() && !list.ends_with(b"\n") {
        list.push(b'\n');
    }
    for (name, folder) in plugins {
        let mut files = Vec::new();
        collect_files(folder, folder, &mut files);
        files.sort();
        for (relative, file) in files {
            match std::fs::read(&file) {
                Ok(bytes) => {
                    let line = serde_json::json!({ "path": format!("plugin/{name}/{relative}"), "size": bytes.len(), "md5": format!("{:x}", md5::compute(&bytes)) });
                    list.extend_from_slice(line.to_string().as_bytes());
                    list.push(b'\n');
                }
                Err(error) => warn!("Plugin {name} asset {} is not served: {error}", file.display()),
            }
        }
    }
    list
}

fn manifest_lines(root: &Path) -> Vec<u8> {
    #[derive(serde::Deserialize)]
    struct Manifest {
        files: Vec<serde_json::Value>,
    }
    let manifest = std::fs::read(root.join("manifest.json")).ok().and_then(|bytes| serde_json::from_slice::<Manifest>(&bytes).ok());
    let Some(manifest) = manifest else {
        warn!("{CHECK_FILE} and manifest.json are missing from {}, /data/check lists only plugin files", root.display());
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
        let app = router(root.to_str().unwrap(), &[]);

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
        let app = router(root.to_str().unwrap(), &[]);

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
        let app = router(root.to_str().unwrap(), &[]);

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
        let app = router(root.to_str().unwrap(), &[("ui_shop".into(), folder)]);

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
}
