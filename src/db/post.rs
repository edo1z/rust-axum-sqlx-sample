use crate::db::get_db_pool;
use crate::models::post::{Post, PostList};
use crate::error::Result;
use anyhow::Context;

pub async fn find_all() -> Result<PostList> {
    let pool = get_db_pool().await;
    let result = sqlx::query_as::<_, Post>("select * from posts")
        .fetch_all(pool)
        .await.context("DB ERROR (find all posts)")?;
    Ok(result)
}
