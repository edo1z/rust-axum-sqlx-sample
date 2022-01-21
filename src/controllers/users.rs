use crate::db;
use crate::models::user::User;
use axum::{http::StatusCode, Json};

pub async fn index() -> Result<Json<Vec<User>>, StatusCode> {
    let users = db::user::find_all().await.map_err(|err| {
        tracing::error!("Error find all users: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(users))
}
