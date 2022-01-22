use crate::controllers;
use axum::{routing::{get, post}, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", get(controllers::root))
        .nest("/users", user_routes())
        .nest("/categories", category_routes())
        .nest("/posts", post_routes())
}

fn user_routes() -> Router {
    Router::new()
        .route("/", get(controllers::users::index))
        .route("/add", post(controllers::users::add))
        .route("/edit", post(controllers::users::edit))
        .route("/delete", post(controllers::users::delete))
}

fn category_routes() -> Router {
    Router::new()
        .route("/", get(controllers::categories::index))
        .route("/add", post(controllers::categories::add))
        .route("/edit", post(controllers::categories::edit))
        .route("/delete", post(controllers::categories::delete))
}

fn post_routes() -> Router {
    Router::new()
        .route("/", get(controllers::posts::index))
        .route("/add", post(controllers::posts::add))
        .route("/edit", post(controllers::posts::edit))
        .route("/delete", post(controllers::posts::delete))
}