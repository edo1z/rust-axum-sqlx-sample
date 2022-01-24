use crate::db;
use crate::error::Result;
use crate::models::user::{CreateUser, UserConditions, UserId, UserList};
use axum::{extract::Query, http::StatusCode, Json};

pub async fn index(Query(conditions): Query<UserConditions>) -> Result<Json<UserList>> {
    let users = db::user::find_all(conditions).await?;
    Ok(Json(users))
}

pub async fn add(Json(user_data): Json<CreateUser>) -> Result<Json<UserId>> {
    let user_id = db::user::add(user_data).await?;
    Ok(Json(user_id))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}
