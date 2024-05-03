use axum::{routing::get, Router};
use axum_server::accept::NoDelayAcceptor;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tokio::join!(serve(app(), 8123),);
}

fn app() -> Router {
    let serve_dir = ServeDir::new("../zola/public")
        .not_found_service(ServeFile::new("../zola/public/404.html"));

    let framedata = ServeDir::new("../../rukaidata/root/framedata")
        .precompressed_gzip()
        .not_found_service(ServeFile::new("../../rukaidata/root/framedata/error.html"));

    let pplus = ServeDir::new("../../rukaidata/root/P+")
        .precompressed_gzip()
        .not_found_service(ServeFile::new("../../rukaidata/root/framedata/error.html"));

    Router::new()
        .route("/foo", get(|| async { "Hi from /foo" }))
        .nest_service("/framedata", framedata)
        .nest_service("/P+", pplus)
        .fallback_service(serve_dir)
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);
    axum_server::bind(addr)
        .acceptor(NoDelayAcceptor)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
