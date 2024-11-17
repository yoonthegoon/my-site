use crate::blog::templates::IndexTemplate;
use crate::templates::HtmlTemplate;
use axum::response::IntoResponse;

pub async fn index_handler() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}
