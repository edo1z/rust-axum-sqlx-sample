use crate::error::Result;
use crate::models::user::{ImgUrl, NewUser, ProfImg, User, UserConditions, UserId, UserList};
use crate::repositories::{user::UserRepo, Repositories};
use std::sync::Arc;

pub async fn search<R: Repositories>(
    repo: Arc<R>,
    conditions: &UserConditions,
) -> Result<UserList> {
    let users = repo.user().find_all(conditions).await?;
    Ok(users)
}

pub async fn view<R: Repositories>(repo: Arc<R>, user_id: i32) -> Result<User> {
    let user = repo.user().find_by_id(user_id).await?;
    Ok(user)
}

pub async fn add<R: Repositories>(repo: Arc<R>, new_user: &NewUser) -> Result<UserId> {
    let user_id = repo.user().add(&new_user).await?;
    Ok(user_id)
}

pub async fn edit_prof_img<R: Repositories>(_repo: Arc<R>, _prof_img: &ProfImg) -> Result<ImgUrl> {
    Ok(ImgUrl {
        url: String::from("https://example.com/hoge.png"),
    })
}

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
