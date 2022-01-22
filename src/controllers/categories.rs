use crate::db;
use crate::models::category::{Category, CategoryConditions, CategoryId, CreateCategory};
use axum::{extract, http::StatusCode, Json};

pub async fn index(
    query: extract::Query<CategoryConditions>,
) -> Result<Json<Vec<Category>>, StatusCode> {
    let category_conditions = query.0;
    let categories = db::category::find_all(category_conditions)
        .await
        .map_err(|err| {
            tracing::error!("Error find all categories: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(categories))
}

pub async fn add(json: extract::Json<CreateCategory>) -> Result<Json<CategoryId>, StatusCode> {
    let category_data = json.0;
    let category_id = db::category::add(category_data).await.map_err(|err| {
        tracing::error!("Error category add: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(category_id))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}
