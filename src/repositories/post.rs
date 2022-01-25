use crate::db::postgres::Db;
use crate::error::Result;
use crate::models::post::{Post, PostList};
use anyhow::Context;
use async_trait::async_trait;

pub struct PostRepo {
    pool: Db,
}
impl PostRepo {
    pub fn new(pool: Db) -> Self {
        Self { pool: pool }
    }
}

#[async_trait]
pub trait PostRepository {
    async fn find_all(&self) -> Result<PostList>;
}

#[async_trait]
impl PostRepository for PostRepo {
    async fn find_all(&self) -> Result<PostList> {
        let result = sqlx::query_as::<_, Post>("select * from posts")
            .fetch_all(&*self.pool)
            .await
            .context("DB ERROR (find all posts)")?;
        Ok(result)
    }
}
