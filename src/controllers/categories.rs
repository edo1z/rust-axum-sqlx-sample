use crate::db;
use crate::models::category::Category;
use axum::{http::StatusCode, Json};

pub async fn index() -> Result<Json<Vec<Category>>, StatusCode> {
    let categories = db::category::find_all().await.map_err(|err| {
        tracing::error!("Error find all categories: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(categories))
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
