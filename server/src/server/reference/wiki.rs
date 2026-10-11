//! `/wiki/<page>` serves the markdown copy of the iRO Wiki Classic kept in `wiki/`, see `docs/reference-api.md`.

use axum::Router;
use axum::extract::Request;
use axum::http::{HeaderValue, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Redirect, Response};
use tower_http::services::ServeDir;

pub fn router(root: &str) -> Router {
    Router::new()
        .nest_service("/wiki", ServeDir::new(root).append_index_html_on_directories(false))
        .layer(middleware::from_fn(root_opens_index))
        .layer(middleware::map_response(markdown_charset))
}

/// `/wiki` has no file of its own; it opens the index page.
async fn root_opens_index(request: Request, next: Next) -> Response {
    if request.uri().path() == "/wiki" {
        return Redirect::permanent("/wiki/index.md").into_response();
    }
    next.run(request).await
}

/// `mime_guess` names markdown without a charset, the pages are UTF-8.
async fn markdown_charset(mut response: Response) -> Response {
    if response.headers().get(header::CONTENT_TYPE).is_some_and(|value| value == "text/markdown") {
        response.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("text/markdown; charset=utf-8"));
    }
    response
}
