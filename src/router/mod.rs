use axum::{routing::get, Router};
use crate::controllers;

pub fn router() -> Router {
    Router::new().route("/", get(controllers::root))
}
