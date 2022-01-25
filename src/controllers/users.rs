use crate::error::Result;
use crate::models::user::{CreateUser, UserConditions, UserId, UserList};
use crate::repositories::{user::UserRepository, RepoExt};
use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    Json,
};

pub async fn index(
    Query(conditions): Query<UserConditions>,
    Extension(repo): RepoExt,
) -> Result<Json<UserList>> {
    let users = repo.user.find_all(&conditions).await?;
    Ok(Json(users))
}

pub async fn add(
    Json(user_data): Json<CreateUser>,
    Extension(repo): RepoExt,
) -> Result<Json<UserId>> {
    let user_id = repo.user.add(&user_data).await?;
    Ok(Json(user_id))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::{router, repositories::user::MockUserRepo, models::user::User};
    // use axum::{body::Body, http::{Request, StatusCode}};
    // use tower::ServiceExt;

    #[tokio::test]
    async fn index() {
        assert_eq!(1, 0);
    }
}
