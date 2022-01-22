use crate::db;
use crate::models::user::{User, UserConditions, CreateUser, UserId};
use axum::{extract, http::StatusCode, Json};

pub async fn index(query: extract::Query<UserConditions>) -> Result<Json<Vec<User>>, StatusCode> {
    let user_conditions = query.0;
    let users = db::user::find_all(user_conditions).await.map_err(|err| {
        tracing::error!("Error find all users: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(users))
}

pub async fn add(json: extract::Json<CreateUser>) -> Result<Json<UserId>, StatusCode> {
    let user_data  = json.0;
    let user_id = db::user::add(user_data).await.map_err(|err| {
        tracing::error!("Error user add: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(user_id))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}