use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub msg: Option<String>,
    pub age: Option<i16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConditions {
    pub name: Option<String>
}