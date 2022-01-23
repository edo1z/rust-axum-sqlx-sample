use crate::db::get_db_pool;
use crate::models::category::{Category, CategoryConditions, CategoryId, CreateCategory};

pub async fn find_all(conditions: CategoryConditions) -> anyhow::Result<Vec<Category>> {
    let pool = get_db_pool().await;
    let mut query = sqlx::query_as::<_, Category>("select * from categories");
    if let Some(name) = conditions.name {
        query = sqlx::query_as::<_, Category>("select * from categories where name LIKE $1")
            .bind(format!("%{}%", name))
    }
    let result = query.fetch_all(pool).await?;
    Ok(result)
}

pub async fn add(category_data: CreateCategory) -> anyhow::Result<CategoryId> {
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
    .await?;
    Ok(row)
}
