mod handlers;
mod templates;

use crate::blog::handlers::index_handler;
use axum::routing::get;
use axum::Router;

pub fn blog_routes() -> Router {
    Router::new().route("/", get(index_handler))
}
