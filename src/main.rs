use crate::blog::{blog_routes, post_detail_handler};
use crate::handlers::index_handler;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

#[cfg(feature = "dev")]
use listenfd::ListenFd;

#[cfg(feature = "dev")]
use tower_http::services::ServeDir;

mod blog;
mod handlers;
mod templates;

#[tokio::main]
async fn main() {
    let app = routes().await;
    let listener = get_listener().await;

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn routes() -> Router {
    let app = Router::new()
        .route("/", get(index_handler))
        .nest("/blog", blog_routes());

    #[cfg(feature = "dev")]
    let app = app.nest_service("/public", ServeDir::new("public"));

    app
}

async fn get_listener() -> TcpListener {
    #[cfg(feature = "dev")]
    {
        let mut listenfd = ListenFd::from_env();
        match listenfd.take_tcp_listener(0).unwrap() {
            Some(listener) => {
                listener.set_nonblocking(true).unwrap();
                return TcpListener::from_std(listener).unwrap();
            }
            None => {}
        }
    }

    TcpListener::bind("127.0.0.1:3000").await.unwrap()
}
