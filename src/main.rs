use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Origin, any};

mod controllers;
mod db;
mod models;
mod router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = router::router().layer(cors());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn cors() -> CorsLayer {
    let swagger_url = "http://localhost:8001";
    CorsLayer::new()
        .allow_origin(Origin::exact(swagger_url.parse().unwrap()))
        .allow_methods(any())
}