mod utils;

use crate::utils::containers::keycloak::{Client, Keycloak, Realm, User};
use axum::http::StatusCode;
use axum_test::TestServerConfig;
use reqwest::redirect::Policy;
use scrounch_backend::app;
use serde_json::{json, Value};
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

#[tokio::test(flavor = "multi_thread")]
async fn basic_login_oidc() {
    let john = User {
        username: "jojo".to_string(),
        email: "john.doe@example.com".to_string(),
        firstname: "john".to_string(),
        lastname: "doe".to_string(),
        password: "jopass".to_string(),
    };

    let basic_client = Client {
        client_id: "scrouch-backend-example-basic".to_string(),
        client_secret: Some("123456".to_string()),
        ..Default::default()
    };

    let realm_name = "test";

    let keycloak = Keycloak::start(vec![Realm {
        name: realm_name.to_string(),
        users: vec![john.clone()],
        clients: vec![basic_client.clone()],
    }])
    .await
    .unwrap();

    let keycloak_url = keycloak.url();
    let issuer = format!("{keycloak_url}/realms/{realm_name}");

    let db_node = Postgres::default().start().await.unwrap();
    let db_url = &format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        db_node.get_host_port_ipv4(5432).await.unwrap()
    );

    let mut arguments = scrounch_backend::Arguments::default();
    arguments.openid_issuer = issuer.clone();
    arguments.openid_client_id = basic_client.client_id;
    arguments.openid_client_secret = basic_client.client_secret;
    arguments.backend_url = "http://localhost:3000".to_string();
    arguments.frontend_url = "http://localhost:5173".to_string();
    arguments.database_url = db_url.to_string();

    let app = app(arguments).await;

    let server = TestServerConfig::builder()
        .save_cookies()
        .http_transport()
        .build_server(app)
        .unwrap();

    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .redirect(Policy::none())
        .build()
        .unwrap();

    // GET /me
    let response = server.get("/me").await;
    response.assert_status(StatusCode::NO_CONTENT);

    // GET /login
    let response = server.get("/login").await;
    response.assert_status(StatusCode::TEMPORARY_REDIRECT);
    let url = utils::extract::extract_location_header_testresponse(response).unwrap();

    // GET keycloak/auth
    let response = client.get(url).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let html = response.text().await.unwrap();
    let url_regex = regex::Regex::new(r#"action="([^"]+)""#).unwrap();
    let url = url_regex.captures(&html).unwrap().get(1).unwrap().as_str();
    let params = [("username", "jojo"), ("password", "jopass")];

    // POST keycloak/auth
    let response = client.post(url).form(&params).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::FOUND);
    let url = utils::extract::extract_location_header_response(response).unwrap();
    let url = url.replace("http://localhost:3000", ""); // Remove http://localhost:3000

    // GET /login-callback
    let response = server.get(&url).await;
    response.assert_status(StatusCode::TEMPORARY_REDIRECT);
    response.assert_header("Location", "http://localhost:3000/login");

    // GET /me
    let response = server.get("/me").await;
    response.assert_status(StatusCode::OK);
    let binding = response.json::<Value>();
    let id = binding.get("id").unwrap();
    response.assert_json(
        &json!({"id": id, "name": "john doe", "email": "john.doe@example.com" , "username": "jojo"}),
    )
}