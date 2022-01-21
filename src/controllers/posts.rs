use crate::db;
use crate::models::post::Post;
use axum::{http::StatusCode, Json};

pub async fn index() -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = db::post::find_all().await.map_err(|err| {
        tracing::error!("Error find all posts: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(posts))
}
