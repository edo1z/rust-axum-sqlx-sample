use crate::db::get_db_pool;
use crate::models::category::Category;

pub async fn find_all() -> Result<Vec<Category>, sqlx::Error> {
    let pool = get_db_pool().await;
    sqlx::query_as::<_, Category>("select * from categories")
        .fetch_all(pool)
        .await
}
