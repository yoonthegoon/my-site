use crate::blog::blog_routes;
use askama_axum::Template;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod blog;
mod error;
mod result;

#[tokio::main]
async fn main() {
    let app = routes().await;
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn routes() -> Router {
    Router::new()
        .route("/", get(|| async { IndexTemplate {} }))
        .nest("/blog", blog_routes())
        .fallback_service(ServeDir::new("public"))
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}
