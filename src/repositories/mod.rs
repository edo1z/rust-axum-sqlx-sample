use crate::db::postgres;
use crate::repositories::{category::CategoryRepo, post::PostRepo, user::UserRepo};
use std::sync::Arc;
use axum::{extract::Extension};

pub mod category;
pub mod post;
pub mod user;

pub type RepoExt = Extension<Arc<Repositories>>;

pub struct Repositories {
    pub user: UserRepo,
    pub category: CategoryRepo,
    pub post: PostRepo,
}

pub async fn create_repositories() -> Repositories {
    let db_pool = Arc::new(postgres::db_connect().await);
    Repositories {
        user: UserRepo::new(db_pool.clone()),
        category: CategoryRepo::new(db_pool.clone()),
        post: PostRepo::new(db_pool.clone()),
    }
}
