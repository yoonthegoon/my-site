use crate::blog::blog_routes;
use crate::handlers::{index_handler, not_found_handler};
use axum::routing::get;
use axum::Router;
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod blog;
mod handlers;
mod templates;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .fallback(not_found_handler)
        .route("/", get(index_handler))
        .nest("/blog", blog_routes());

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
