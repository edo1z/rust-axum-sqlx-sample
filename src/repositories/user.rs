use crate::db::postgres::Db;
use crate::error::Result;
use crate::models::user::{CreateUser, User, UserConditions, UserId, UserList};
use anyhow::Context;
use async_trait::async_trait;

pub struct UserRepo {
    pool: Db,
}
impl UserRepo {
    pub fn new(pool: Db) -> Self {
        Self { pool: pool }
    }
}

#[async_trait]
pub trait UserRepository {
    async fn find_all(&self, conditions: &UserConditions) -> Result<UserList>;
    async fn add(&self, user_data: &CreateUser) -> Result<UserId>;
}

#[async_trait]
impl UserRepository for UserRepo {
    async fn find_all(&self, conditions: &UserConditions) -> Result<UserList> {
        let mut query = sqlx::query_as::<_, User>("select * from users");
        if let Some(name) = &conditions.name {
            query = sqlx::query_as::<_, User>("select * from users where name LIKE $1")
                .bind(format!("%{}%", name))
        }
        let result = query
            .fetch_all(&*self.pool)
            .await
            .context("DB ERROR (find all users)")?;
        Ok(result)
    }

    async fn add(&self, user_data: &CreateUser) -> Result<UserId> {
        let row = sqlx::query_as::<_, UserId>(
            r#"
            INSERT INTO users (name, msg, age)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(&user_data.name)
        .bind(&user_data.msg)
        .bind(&user_data.age)
        .fetch_one(&*self.pool)
        .await
        .context("DB ERROR (create user)")?;
        Ok(row)
    }
}
