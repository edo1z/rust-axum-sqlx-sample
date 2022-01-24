use crate::db;
use crate::error::Result;
use crate::models::post::{PostList};
use axum::{http::StatusCode, Json};

pub async fn index() -> Result<Json<PostList>> {
    let posts = db::post::find_all().await?;
    Ok(Json(posts))
}

pub async fn add() -> StatusCode {
    StatusCode::OK
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}
