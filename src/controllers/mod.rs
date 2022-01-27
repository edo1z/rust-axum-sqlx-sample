pub mod categories;
pub mod posts;
pub mod users;

pub async fn root() -> &'static str {
    "hello"
}

#[cfg(test)]
mod tests {
    use crate::router;
    use crate::test::request;
    use axum::{body::Body, http::StatusCode};

    #[tokio::test]
    async fn index() {
        let app = router::router();
        let response = request(app, "/", Body::empty()).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
