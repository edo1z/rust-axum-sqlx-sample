use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub msg: Option<String>,
    pub age: Option<i16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub name: String,
    pub msg: Option<String>,
    pub age: Option<i16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConditions {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct UserId {
    pub id: i32
}