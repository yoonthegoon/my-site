mod handlers;
mod models;
mod schema;
mod templates;

use askama_axum::IntoResponse;
use axum::http::HeaderValue;
use axum::response::Response;
use crate::blog::handlers::{post_detail_handler, post_list_handler};
use axum::routing::get;
use axum::Router;

pub fn blog_routes() -> Router {
    Router::new()
        .route("/", get(post_list_handler))
        .route("/:slug", get(post_detail_handler))
}
