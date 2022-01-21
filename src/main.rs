use std::net::SocketAddr;

mod controllers;
mod db;
mod models;
mod router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = router::router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
