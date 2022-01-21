use crate::db::get_db_pool;
use crate::models::post::Post;

pub async fn find_all() -> Result<Vec<Post>, sqlx::Error> {
    let pool = get_db_pool().await;
    sqlx::query_as::<_, Post>("select * from posts")
        .fetch_all(pool)
        .await
}
