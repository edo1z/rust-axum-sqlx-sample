use crate::db::postgres::Db;
use crate::error::Result;
use crate::models::category::{
    Category, CategoryConditions, CategoryId, CategoryList, CreateCategory,
};
use anyhow::Context;
use async_trait::async_trait;

pub struct CategoryRepo {
    pool: Db,
}
impl CategoryRepo {
    pub fn new(pool: Db) -> Self {
        Self { pool: pool }
    }
}

#[async_trait]
pub trait CategoryRepository {
    async fn find_all(&self, conditions: &CategoryConditions) -> Result<CategoryList>;
    async fn add(&self, category_data: &CreateCategory) -> Result<CategoryId>;
}

#[async_trait]
impl CategoryRepository for CategoryRepo {
    async fn find_all(&self, conditions: &CategoryConditions) -> Result<CategoryList> {
        let mut query = sqlx::query_as::<_, Category>("select * from categories");
        if let Some(name) = &conditions.name {
            query = sqlx::query_as::<_, Category>("select * from categories where name LIKE $1")
                .bind(format!("%{}%", name))
        }
        let result = query
            .fetch_all(&*self.pool)
            .await
            .context("DB ERROR (find add categories)")?;
        Ok(result)
    }

    async fn add(&self, category_data: &CreateCategory) -> Result<CategoryId> {
        let row = sqlx::query_as::<_, CategoryId>(
            r#"
        INSERT INTO categories (name)
        VALUES ($1)
        RETURNING id
        "#,
        )
        .bind(&category_data.name)
        .fetch_one(&*self.pool)
        .await
        .context("DB ERROR (create category)")?;
        Ok(row)
    }
}
