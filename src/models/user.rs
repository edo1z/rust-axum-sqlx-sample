use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub msg: Option<String>,
    pub age: Option<i16>,
}

pub type UserList = Vec<User>;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
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
    pub id: i32,
}

#[derive(Serialize)]
pub struct ProfImg {
    pub user_id: i32,
    pub prof_img: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ImgUrl {
    pub url: String,
}
