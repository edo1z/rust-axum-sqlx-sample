use std::net::SocketAddr;

mod bootstrap;
mod controllers;
mod db;
mod error;
mod models;
mod repositories;
mod router;
#[cfg(test)]
mod tests;
mod usecases;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    let app = bootstrap::create_app().await;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
