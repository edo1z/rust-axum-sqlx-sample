use crate::db::get_db_pool;
use crate::models::user::{CreateUser, User, UserConditions, UserId, UserList};
use crate::error::Result;
use anyhow::Context;

pub async fn find_all(conditions: UserConditions) -> Result<UserList> {
    let pool = get_db_pool().await;
    let mut query = sqlx::query_as::<_, User>("select * from users");
    if let Some(name) = conditions.name {
        query = sqlx::query_as::<_, User>("select * from users where name LIKE $1")
            .bind(format!("%{}%", name))
    }
    let result = query.fetch_all(pool).await.context("DB ERROR (find all users)")?;
    Ok(result)
}

pub async fn add(user_data: CreateUser) -> Result<UserId> {
    let pool = get_db_pool().await;
    let row = sqlx::query_as::<_, UserId>(
        r#"
        INSERT INTO users (name, msg, age)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
    )
    .bind(user_data.name)
    .bind(user_data.msg)
    .bind(user_data.age)
    .fetch_one(pool)
    .await.context("DB ERROR (create user)")?;
    Ok(row)
}
