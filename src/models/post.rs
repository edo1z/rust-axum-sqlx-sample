use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub user_id: u32,
    pub category_id: u32,
    pub title: String,
    pub content: String,
}
