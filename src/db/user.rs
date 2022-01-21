use crate::db::get_db_pool;
use crate::models::user::User;

pub async fn find_all() -> Result<Vec<User>, sqlx::Error> {
    let pool = get_db_pool().await;
    sqlx::query_as::<_, User>("select * from users")
        .fetch_all(pool)
        .await
}
