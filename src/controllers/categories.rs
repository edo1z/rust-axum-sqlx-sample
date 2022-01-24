use crate::db;
use crate::error::Result;
use crate::models::category::{CategoryList, CategoryConditions, CategoryId, CreateCategory};
use axum::{extract::Query, http::StatusCode, Json};

pub async fn index(
    Query(conditions): Query<CategoryConditions>
) -> Result<Json<CategoryList>> {
    let categories = db::category::find_all(conditions).await?;
    Ok(Json(categories))
}

pub async fn add(Json(category_data): Json<CreateCategory>) -> Result<Json<CategoryId>> {
    let category_id = db::category::add(category_data).await?;
    Ok(Json(category_id))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}
