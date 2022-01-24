pub mod categories;
pub mod posts;
pub mod users;

pub async fn root() -> &'static str {
    "hello"
}

#[cfg(test)]
mod tests {
    use crate::router;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn index() {
        let app = router::router();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
