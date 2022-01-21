use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
}
