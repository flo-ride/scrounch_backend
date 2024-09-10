use axum::{body::Body, extract::Request, http::StatusCode};
use http_body_util::BodyExt;
use scrounch_backend::app;
use tower::util::ServiceExt;

#[tokio::test]
async fn basic_status_test() {
    let app = app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/status")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();

    assert_eq!(body, "UP");
}
