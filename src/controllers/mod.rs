pub mod categories;
pub mod posts;
pub mod users;

pub async fn root() -> &'static str {
    "hello"
}
