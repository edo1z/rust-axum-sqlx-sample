use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
}
