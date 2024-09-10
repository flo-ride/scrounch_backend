use axum::{body::Body, extract::Request, http::StatusCode};
use http_body_util::BodyExt;
use scrounch_backend::app;
use serde_json::{json, Value};
use tower::util::ServiceExt;

#[tokio::test]
async fn basic_swagger_test() {
    let arguments = scrounch_backend::Arguments::default();

    let app = app(arguments).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/swagger-ui")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    if cfg!(debug_assertions) {
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
    } else {
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api-docs/openapi.json")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    if cfg!(debug_assertions) {
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        // Simple text for beeing sure that openapi is working, don't forget to bump version
        assert_eq!(*body.get("openapi").unwrap(), json!("3.0.3"));
    } else {
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
