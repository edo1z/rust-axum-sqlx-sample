use crate::repositories::create_repositories;
use crate::router::router;
use axum::{http::header::CONTENT_TYPE, AddExtensionLayer, Router};
use std::sync::Arc;
use tower_http::cors::{any, CorsLayer, Origin};

pub fn cors() -> CorsLayer {
    let swagger_url = "http://localhost:8001";
    CorsLayer::new()
        .allow_origin(Origin::exact(swagger_url.parse().unwrap()))
        .allow_methods(any())
        .allow_headers(vec![CONTENT_TYPE])
}

pub async fn create_app() -> Router {
    let repositories = Arc::new(create_repositories().await);
    router()
        .layer(cors())
        .layer(AddExtensionLayer::new(repositories))
}
