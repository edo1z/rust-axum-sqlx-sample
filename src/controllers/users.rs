use crate::db;
use crate::models::user::{User, UserConditions};
use axum::{extract::Query, http::StatusCode, Json};

pub async fn index(query: Query<UserConditions>) -> Result<Json<Vec<User>>, StatusCode> {
    let user_conditions = query.0;
    let users = db::user::find_all(user_conditions).await.map_err(|err| {
        tracing::error!("Error find all users: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(users))
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