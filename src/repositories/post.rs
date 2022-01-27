use crate::db::postgres::Db;
use crate::error::Result;
use crate::models::post::{Post, PostList};
use anyhow::Context;
use async_trait::async_trait;
use mockall::automock;

pub struct PostRepoImpl {
    pool: Db,
}
impl PostRepoImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool: pool }
    }
}

#[automock]
#[async_trait]
pub trait PostRepo {
    async fn find_all(&self) -> Result<PostList>;
}

#[async_trait]
impl PostRepo for PostRepoImpl {
    async fn find_all(&self) -> Result<PostList> {
        let result = sqlx::query_as::<_, Post>("select * from posts")
            .fetch_all(&*self.pool)
            .await
            .context("DB ERROR (find all posts)")?;
        Ok(result)
    }
}
