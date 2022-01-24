use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

pub type CategoryList = Vec<Category>;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCategory {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryConditions {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct CategoryId {
    pub id: i32,
}
