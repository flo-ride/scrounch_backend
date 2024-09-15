mod utils;

use axum::{body::Body, extract::Request, http::StatusCode};
use http_body_util::BodyExt;
use scrounch_backend::app;
use serde_json::{json, Value};
use tower::util::ServiceExt;

use crate::utils::containers::keycloak::{Client, Keycloak, Realm};

#[tokio::test(flavor = "multi_thread")]
async fn basic_swagger_test() {
    let realm_name = "master";
    let basic_client = Client {
        client_id: "scrouch-backend-example-basic".to_string(),
        client_secret: Some("123456".to_string()),
        ..Default::default()
    };

    let keycloak = Keycloak::start(vec![Realm {
        name: realm_name.to_string(),
        users: vec![],
        clients: vec![basic_client.clone()],
    }])
    .await
    .unwrap();

    let url = keycloak.url();

    let mut arguments = scrounch_backend::Arguments::default();
    arguments.openid_issuer = format!("{url}/realms/{realm_name}");
    arguments.openid_client_id = basic_client.client_id;
    arguments.openid_client_secret = basic_client.client_secret;
    arguments.backend_base_url = "http://localhost:3000".to_string();
    arguments.frontend_base_url = "http://localhost:5173".to_string();

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

#[tokio::test]
async fn basic_status_test() {
    let realm_name = "master";
    let basic_client = Client {
        client_id: "scrouch-backend-example-basic".to_string(),
        client_secret: Some("123456".to_string()),
        ..Default::default()
    };

    let keycloak = Keycloak::start(vec![Realm {
        name: realm_name.to_string(),
        users: vec![],
        clients: vec![basic_client.clone()],
    }])
    .await
    .unwrap();

    let url = keycloak.url();

    let mut arguments = scrounch_backend::Arguments::default();
    arguments.openid_issuer = format!("{url}/realms/{realm_name}");
    arguments.openid_client_id = basic_client.client_id;
    arguments.openid_client_secret = basic_client.client_secret;
    arguments.backend_base_url = "http://localhost:3000".to_string();
    arguments.frontend_base_url = "http://localhost:5173".to_string();

    let app = app(arguments).await;

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
