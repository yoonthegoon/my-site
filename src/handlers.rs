use crate::templates::IndexTemplate;
use crate::templates::{HtmlTemplate, NotFoundTemplate};
use axum::response::IntoResponse;

pub async fn index_handler() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

pub async fn not_found_handler() -> impl IntoResponse {
    let template = NotFoundTemplate {};
    HtmlTemplate(template)
}
