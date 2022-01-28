use crate::error::Result;
use crate::models::category::{
    Category, CategoryConditions, CategoryId, CategoryList, CreateCategory,
};
use crate::repositories::{category::CategoryRepo, RepoExt, Repositories};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};

pub async fn index(
    Query(conditions): Query<CategoryConditions>,
    Extension(repo): RepoExt,
) -> Result<Json<CategoryList>> {
    let categories = repo.category().find_all(&conditions).await?;
    Ok(Json(categories))
}

pub async fn view(
    Path(category_id): Path<i32>,
    Extension(repo): RepoExt,
) -> Result<Json<Category>> {
    let category = repo.category().find_by_id(category_id).await?;
    Ok(Json(category))
}

pub async fn add(
    Json(category_data): Json<CreateCategory>,
    Extension(repo): RepoExt,
) -> Result<Json<CategoryId>> {
    let category_id = repo.category().add(&category_data).await?;
    Ok(Json(category_id))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}
