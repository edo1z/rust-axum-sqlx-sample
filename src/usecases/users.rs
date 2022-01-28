use crate::error::Result;
use crate::models::user::{UserConditions, UserList};
use crate::repositories::{user::UserRepo, Repositories};
use std::sync::Arc;

pub async fn search<R: Repositories>(
    repo: Arc<R>,
    conditions: &UserConditions,
) -> Result<UserList> {
    let users = repo.user().find_all(conditions).await?;
    Ok(users)
}

// pub async fn view() {}

// pub async fn add() {}

// pub async fn edit() {}

// pub async fn delete() {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{fixture::user::users_fixture, repositories::create_repositories_for_test};

    #[tokio::test]
    async fn test_search() {
        let mut mock_repo_impl = create_repositories_for_test().await;
        mock_repo_impl
            .user
            .expect_find_all()
            .returning(|_| Ok(users_fixture(5)));
        let conditions = UserConditions { name: None };
        let users = search(Arc::new(mock_repo_impl), &conditions).await.unwrap();
        assert_eq!(users.len(), 5);
    }
}
