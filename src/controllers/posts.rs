use crate::error::Result;
use crate::models::post::PostList;
use crate::repositories::{post::PostRepo, RepoExt, Repositories};
use axum::{extract::Extension, http::StatusCode, Json};

pub async fn index(Extension(repo): RepoExt) -> Result<Json<PostList>> {
    let posts = repo.post().find_all().await?;
    Ok(Json(posts))
}

pub async fn add() -> StatusCode {
    StatusCode::OK
}

pub async fn view() -> StatusCode {
    StatusCode::OK
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}
