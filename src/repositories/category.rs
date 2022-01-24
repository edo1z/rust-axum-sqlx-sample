use crate::error::Result;
use crate::models::category::{
    Category, CategoryConditions, CategoryId, CategoryList, CreateCategory,
};
use crate::repositories::get_db_pool;
use anyhow::Context;

pub struct CategoryRepo {}
impl CategoryRepo {
    pub async fn find_all(conditions: CategoryConditions) -> Result<CategoryList> {
        let pool = get_db_pool().await;
        let mut query = sqlx::query_as::<_, Category>("select * from categories");
        if let Some(name) = conditions.name {
            query = sqlx::query_as::<_, Category>("select * from categories where name LIKE $1")
                .bind(format!("%{}%", name))
        }
        let result = query
            .fetch_all(pool)
            .await
            .context("DB ERROR (find add categories)")?;
        Ok(result)
    }

    pub async fn add(category_data: CreateCategory) -> Result<CategoryId> {
        let pool = get_db_pool().await;
        let row = sqlx::query_as::<_, CategoryId>(
            r#"
        INSERT INTO categories (name)
        VALUES ($1)
        RETURNING id
        "#,
        )
        .bind(category_data.name)
        .fetch_one(pool)
        .await
        .context("DB ERROR (create category)")?;
        Ok(row)
    }
}
