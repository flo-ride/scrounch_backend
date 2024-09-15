mod utils;

use crate::utils::containers::keycloak::{Client, Keycloak, Realm, User};
use axum::{body::Body, extract::Request, http::StatusCode};
use scrounch_backend::{app, Arguments};
use tower::ServiceExt;

#[tokio::test(flavor = "multi_thread")]
async fn basic_login_oidc() {
    let john = User {
        username: "john".to_string(),
        email: "john.doe@example.com".to_string(),
        firstname: "john".to_string(),
        lastname: "doe".to_string(),
        password: "jojo".to_string(),
    };

    let basic_client = Client {
        client_id: "scrouch-backend-example-basic".to_string(),
        client_secret: Some("123456".to_string()),
        ..Default::default()
    };

    let realm_name = "master";

    let keycloak = Keycloak::start(vec![Realm {
        name: realm_name.to_string(),
        users: vec![john.clone()],
        clients: vec![basic_client.clone()],
    }])
    .await
    .unwrap();

    let url = keycloak.url();
    let issuer = format!("{url}/realms/{realm_name}");

    let mut arguments = Arguments::default();
    arguments.openid_issuer = issuer.clone();
    arguments.openid_client_id = basic_client.client_id.clone();
    arguments.openid_client_secret = basic_client.client_secret.clone();
    arguments.backend_base_url = "http://localhost:3000".to_string();
    arguments.frontend_base_url = "http://localhost:5173".to_string();

    let app = app(arguments).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/login")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);

    let client_id = basic_client.client_id.clone();
    let redirect_uri = "http%3A%2F%2Flocalhost%3A3000%2Flogin";

    let redirect_regex = regex::Regex::new(&format!(
        r"{issuer}/protocol/openid-connect/auth\?response_type=code&client_id={client_id}&state=[a-zA-Z0-9_-]+&code_challenge=[a-zA-Z0-9_-]+&code_challenge_method=[a-zA-Z0-9_-]+&redirect_uri={redirect_uri}&scope=openid\+email\+profile&nonce=[a-zA-Z0-9_-]+"
    )).unwrap();
    let redirect = response
        .headers()
        .get("Location")
        .unwrap()
        .to_str()
        .unwrap();

    assert!(redirect_regex.is_match(redirect));
}
