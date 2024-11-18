use crate::templates::HtmlTemplate;
use crate::templates::IndexTemplate;
use axum::response::IntoResponse;

pub async fn index_handler() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}
