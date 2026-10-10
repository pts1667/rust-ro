//! Read-only HTTP server that publishes the extracted client data under `/data`.
//!
//! The tree is built by `tools/client-data-setup` from an installed kRO client: every resource the
//! client can ask for is extracted from the GRF archives, maps are narrowed to the ones the server
//! serves, and the loose `BGM` folder is copied along. Paths are the archive names the client uses,
//! lowercased, with the leading `data/` stripped (`sprite/…`, `texture/…`, `prontera.rsw`, `bgm/…`),
//! which is the same layout the client's `data_dir` option expects. The setup stores compressible
//! files only as `<name>.zst`, which `ServeDir` sends as-is with `Content-Encoding: zstd` when the
//! request accepts it; already-compressed formats stay identity so range requests keep working.

use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use tower_http::services::ServeDir;
use tracing::{error, info};

use crate::server::Server;

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
        let app = router(&config.root);
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
pub fn router(root: &str) -> Router {
    Router::new().nest_service(
        "/data",
        ServeDir::new(root)
            .append_index_html_on_directories(false)
            .precompressed_zstd(),
    )
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
        let app = router(root.to_str().unwrap());

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
    async fn serves_compressed_only_files_and_falls_back_to_identity() {
        let root = test_root("zstd");
        // Compressible files exist only as `.zst`: `poring.spr` from the fixture is stored as-is, and
        // `only-zstd.spr` has no identity file.
        let compressed = zstd::bulk::compress(b"SPR", 3).unwrap();
        std::fs::write(root.join("sprite/only-zstd.spr.zst"), &compressed).unwrap();
        let app = router(root.to_str().unwrap());

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
}
