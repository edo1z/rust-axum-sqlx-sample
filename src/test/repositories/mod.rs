use crate::repositories::{
    category::MockCategoryRepo as MockCategoryRepoImpl, post::MockPostRepo as MockPostRepoImpl,
    user::MockUserRepo as MockUserRepoImpl, Repositories,
};

pub async fn create_repositories_for_test() -> MockRepoImpls {
    MockRepoImpls::new(
        MockUserRepoImpl::new(),
        MockCategoryRepoImpl::new(),
        MockPostRepoImpl::new(),
    )
}

#[derive(Debug)]
pub struct MockRepoImpls {
    pub user: MockUserRepoImpl,
    pub category: MockCategoryRepoImpl,
    pub post: MockPostRepoImpl,
}
impl MockRepoImpls {
    pub fn new(
        mock_user_repo_impl: MockUserRepoImpl,
        mock_category_repo_impl: MockCategoryRepoImpl,
        mock_post_repo_impl: MockPostRepoImpl,
    ) -> Self {
        Self {
            user: mock_user_repo_impl,
            category: mock_category_repo_impl,
            post: mock_post_repo_impl,
        }
    }
}
impl Repositories for MockRepoImpls {
    type UserRepoImpl = MockUserRepoImpl;
    type CategoryRepoImpl = MockCategoryRepoImpl;
    type PostRepoImpl = MockPostRepoImpl;
    fn user(&self) -> &Self::UserRepoImpl {
        &self.user
    }
    fn category(&self) -> &Self::CategoryRepoImpl {
        &self.category
    }
    fn post(&self) -> &Self::PostRepoImpl {
        &self.post
    }
}
