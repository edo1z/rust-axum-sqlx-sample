use crate::db::get_db_pool;
use crate::models::user::{CreateUser, User, UserConditions};

pub async fn find_all(conditions: UserConditions) -> Result<Vec<User>, sqlx::Error> {
    let pool = get_db_pool().await;
    let mut query = sqlx::query_as::<_, User>("select * from users");
    if let Some(name) = conditions.name {
        query = sqlx::query_as::<_, User>("select * from users where name LIKE $1")
            .bind(format!("%{}%", name))
    }
    query.fetch_all(pool).await
}

pub async fn add(user: CreateUser) -> Result<(), sqlx::Error> {
    let pool = get_db_pool().await;
    println!("{:?}", user);
    Ok(())
}
