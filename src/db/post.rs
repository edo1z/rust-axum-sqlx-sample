use crate::db::get_db_pool;
use crate::models::post::Post;

pub async fn find_all() -> anyhow::Result<Vec<Post>> {
    let pool = get_db_pool().await;
    let result = sqlx::query_as::<_, Post>("select * from posts")
        .fetch_all(pool)
        .await?;
    Ok(result)
}
