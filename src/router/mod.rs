use crate::controllers::{categories, posts, root, users};
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/users", user_routes())
        .nest("/categories", category_routes())
        .nest("/posts", post_routes())
}

fn user_routes() -> Router {
    Router::new()
        .route("/", get(users::index))
        .route("/add", post(users::add))
        .route("/edit", post(users::edit))
        .route("/delete", post(users::delete))
}

fn category_routes() -> Router {
    Router::new()
        .route("/", get(categories::index))
        .route("/add", post(categories::add))
        .route("/edit", post(categories::edit))
        .route("/delete", post(categories::delete))
}

fn post_routes() -> Router {
    Router::new()
        .route("/", get(posts::index))
        .route("/add", post(posts::add))
        .route("/edit", post(posts::edit))
        .route("/delete", post(posts::delete))
}
