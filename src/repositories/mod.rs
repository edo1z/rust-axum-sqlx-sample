use crate::db::postgres;
use crate::repositories::{
    category::{CategoryRepo, CategoryRepoImpl},
    post::{PostRepo, PostRepoImpl},
    user::{UserRepo, UserRepoImpl},
};
use axum::extract::Extension;
use std::sync::Arc;

pub mod category;
pub mod post;
pub mod user;

pub type RepoExt = Extension<Arc<RepoImpls>>;

pub async fn create_repositories() -> RepoImpls {
    let db_pool = Arc::new(postgres::db_connect().await);
    RepoImpls::new(
        UserRepoImpl::new(db_pool.clone()),
        CategoryRepoImpl::new(db_pool.clone()),
        PostRepoImpl::new(db_pool.clone()),
    )
}

pub struct RepoImpls {
    pub user: UserRepoImpl,
    pub category: CategoryRepoImpl,
    pub post: PostRepoImpl,
}
impl RepoImpls {
    pub fn new(
        user_repo_impl: UserRepoImpl,
        category_repo_impl: CategoryRepoImpl,
        post_repo_impl: PostRepoImpl,
    ) -> Self {
        Self {
            user: user_repo_impl,
            category: category_repo_impl,
            post: post_repo_impl,
        }
    }
}

pub trait Repositories {
    type UserRepoImpl: UserRepo;
    type CategoryRepoImpl: CategoryRepo;
    type PostRepoImpl: PostRepo;
    fn user(&self) -> &Self::UserRepoImpl;
    fn category(&self) -> &Self::CategoryRepoImpl;
    fn post(&self) -> &Self::PostRepoImpl;
}
impl Repositories for RepoImpls {
    type UserRepoImpl = UserRepoImpl;
    type CategoryRepoImpl = CategoryRepoImpl;
    type PostRepoImpl = PostRepoImpl;
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
