//! Read-only reference for people and programs that plan around the game: `/db` answers what the server knows about an item,
//! a monster, a quest or another category as JSON, `/wiki` serves the iRO Wiki Classic copy. Nothing here needs a session or
//! the game loop, the data is loaded at boot.
mod db;
mod wiki;

use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, warn};

use crate::server::Server;

const SHUTDOWN_POLL: Duration = Duration::from_millis(500);
const API_THREADS: usize = 1;

/// Serves the reference until the server stops. Runs on its own runtime, so a slow client can never starve the game.
pub fn serve(server: Arc<Server>) {
    let config = &server.configuration.reference;
    if !std::path::Path::new(&config.wiki_root).is_dir() {
        warn!("The wiki is not served, {} is not a directory: copy the wiki pages there", config.wiki_root);
    }
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(API_THREADS)
        .thread_name("reference")
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            error!("The reference is disabled, its runtime failed to start: {error}");
            return;
        }
    };
    runtime.block_on(async {
        let listener = match tokio::net::TcpListener::bind((config.host.as_str(), config.port)).await {
            Ok(listener) => listener,
            Err(error) => {
                error!("The reference can not listen on {}:{}: {error}", config.host, config.port);
                return;
            }
        };
        info!("Reference is served on http://{}:{}/db and /wiki", config.host, config.port);
        let app = router(&config.wiki_root);
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
                    error!("The reference stopped: {error}");
                }
            }
            _ = stopped => {}
        }
    });
    runtime.shutdown_background();
}

/// Readable from any origin, so browser pages can use it: everything here is public.
pub fn router(wiki_root: &str) -> Router {
    db::router()
        .merge(wiki::router(wiki_root))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([Method::GET, Method::HEAD]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode, header};
    use tower::ServiceExt;

    fn wiki_root(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("rust_ro_reference_wiki_test_{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("images")).unwrap();
        std::fs::write(dir.join("index.md"), "# Wiki\n\nA page with \u{00e9}.\n").unwrap();
        std::fs::write(dir.join("AGI.md"), "# AGI\n").unwrap();
        std::fs::write(dir.join("images/1.png"), b"PNG").unwrap();
        dir
    }

    fn get(uri: &str) -> Request<Body> {
        Request::builder().uri(uri).body(Body::empty()).unwrap()
    }

    #[tokio::test]
    async fn wiki_pages_are_markdown_in_utf8_and_the_root_opens_the_index() {
        let root = wiki_root("pages");
        let app = router(root.to_str().unwrap());

        let response = app.clone().oneshot(get("/wiki/AGI.md")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get(header::CONTENT_TYPE).unwrap(), "text/markdown; charset=utf-8");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"# AGI\n");

        let response = app.clone().oneshot(get("/wiki/images/1.png")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let response = app.clone().oneshot(get("/wiki")).await.unwrap();
        assert!(response.status().is_redirection());
        assert_eq!(response.headers().get(header::LOCATION).unwrap(), "/wiki/index.md");

        let response = app.oneshot(get("/wiki/../Cargo.toml")).await.unwrap();
        assert_ne!(response.status(), StatusCode::OK, "files outside the wiki must not be served");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn any_origin_reads_the_db_and_the_wiki() {
        let root = wiki_root("cors");
        let app = router(root.to_str().unwrap());
        for uri in ["/wiki/AGI.md", "/db/nothing-here"] {
            let request = Request::builder().uri(uri).header(header::ORIGIN, "http://web.example").body(Body::empty()).unwrap();
            let response = app.clone().oneshot(request).await.unwrap();
            assert_eq!(response.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(), "*", "{uri}");
        }
        let request = Request::builder()
            .method("OPTIONS")
            .uri("/db/items")
            .header(header::ORIGIN, "http://web.example")
            .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert!(response.status().is_success());
        assert!(response.headers().get(header::ACCESS_CONTROL_ALLOW_METHODS).unwrap().to_str().unwrap().contains("GET"));

        let _ = std::fs::remove_dir_all(&root);
    }
}
