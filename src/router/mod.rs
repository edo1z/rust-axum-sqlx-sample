use axum::{routing::get, Router};
use crate::controllers;

pub fn router() -> Router {
    Router::new()
        .route("/", get(controllers::root))
        .route("/users", get(controllers::users::index))
        .route("/categories", get(controllers::categories::index))
        .route("/posts", get(controllers::posts::index))
}
