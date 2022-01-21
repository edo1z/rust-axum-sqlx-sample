pub mod users;
pub mod categories;
pub mod posts;

pub async fn root() -> &'static str {
    "hello"
}