use crate::error::Result;
use crate::models::user::{CreateUser, UserConditions, UserId, UserList};
use crate::repositories::{user::UserRepo, RepoExt, Repositories};
use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    Json,
};

pub async fn index(
    Query(conditions): Query<UserConditions>,
    Extension(repo): RepoExt,
) -> Result<Json<UserList>> {
    let users = repo.user().find_all(&conditions).await?;
    Ok(Json(users))
}

pub async fn add(
    Json(user_data): Json<CreateUser>,
    Extension(repo): RepoExt,
) -> Result<Json<UserId>> {
    let user_id = repo.user().add(&user_data).await?;
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
    use super::*;
    use crate::test::{
        fixture::user::users_fixture,
        repositories::create_repositories_for_test
    };

    #[tokio::test]
    async fn test_index() {
        let mut repo = create_repositories_for_test().await;
        repo
            .user
            .expect_find_all()
            .returning(|_| Ok(users_fixture(5)));
        let conditions = UserConditions{name:None};
        let users = repo.user().find_all(&conditions).await.unwrap();
        assert_eq!(users.len(), 5);
    }
}
