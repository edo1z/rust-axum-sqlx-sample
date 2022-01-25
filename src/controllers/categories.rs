use crate::error::Result;
use crate::models::category::{CategoryConditions, CategoryId, CategoryList, CreateCategory};
use crate::repositories::{category::CategoryRepository, RepoExt};
use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    Json,
};

pub async fn index(
    Query(conditions): Query<CategoryConditions>,
    Extension(repo): RepoExt,
) -> Result<Json<CategoryList>> {
    let categories = repo.category.find_all(&conditions).await?;
    Ok(Json(categories))
}

pub async fn add(
    Json(category_data): Json<CreateCategory>,
    Extension(repo): RepoExt,
) -> Result<Json<CategoryId>> {
    let category_id = repo.category.add(&category_data).await?;
    Ok(Json(category_id))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}
