use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub category_id: i32,
    pub title: String,
    pub content: String,
}
