use axum::http::StatusCode;
use axum_test::{TestServer, TestServerBuilder};
use containers::keycloak::{Keycloak, Realm, User};
use futures::future::join_all;
use scrounch_backend::app;
use testcontainers::{runners::AsyncRunner, ContainerAsync};
use testcontainers_modules::{minio::MinIO, postgres::Postgres};

use crate::utils;

pub mod containers;
pub mod extract;
pub mod generation;

#[allow(dead_code)]
pub async fn create_basic_session(
    realm: Realm,
) -> (
    TestServer,
    Vec<std::string::String>,
    (Keycloak, ContainerAsync<Postgres>, ContainerAsync<MinIO>),
) {
    let keycloak = Keycloak::start(vec![Realm {
        name: realm.name.clone(),
        clients: realm.clients.clone(),
        users: vec![],
    }])
    .await
    .unwrap();

    let ids = join_all(
        realm
            .users
            .iter()
            .map(|x| async { keycloak.create_user(x, &realm.name).await }),
    )
    .await;

    let keycloak_url = keycloak.url();
    let issuer = format!("{keycloak_url}/realms/{}", realm.name);

    let db_node = Postgres::default().start().await.unwrap();
    let db_url = &format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        db_node.get_host_port_ipv4(5432).await.unwrap()
    );

    let minio_node = MinIO::default().start().await.unwrap();
    let minio_url = &format!(
        "http://localhost:{}",
        minio_node.get_host_port_ipv4(9000).await.unwrap()
    );
    let minio_user = "minioadmin";
    let minio_pass = "minioadmin";

    let mut arguments = scrounch_backend::Arguments::default();
    arguments.openid_issuer = issuer.clone();
    arguments.openid_client_id = realm.clients[0].client_id.clone();
    arguments.openid_client_secret = realm.clients[0].client_secret.clone();
    arguments.backend_url = "http://localhost:3000".to_string();
    arguments.frontend_url = "http://localhost:5173".to_string();
    arguments.database_url = db_url.to_string();
    arguments.aws_access_key_id = minio_user.to_string();
    arguments.aws_secret_access_key = minio_pass.to_string();
    arguments.aws_endpoint_url = minio_url.to_string();
    arguments.aws_s3_bucket = "miniobucket".to_string();

    let app = app(arguments).await;

    let server = TestServerBuilder::new()
        .mock_transport()
        .build(app)
        .unwrap();

    (server, ids, (keycloak, db_node, minio_node))
}

#[allow(dead_code)]
pub async fn create_user_session(
    server: &mut TestServer,
    user: User,
) -> tower_sessions::cookie::Cookie<'static> {
    server.save_cookies();
    server.clear_cookies();

    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
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
    let params = [("username", user.username), ("password", user.password)];

    // POST keycloak/auth
    let response = client.post(url).form(&params).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::FOUND);
    let url = utils::extract::extract_location_header_response(response).unwrap();
    let url = url.replace("http://localhost:3000", "");

    // GET /login-callback
    let response = server.get(&url).await;
    response.assert_status(StatusCode::TEMPORARY_REDIRECT);
    response.assert_header("Location", "http://localhost:3000/login");
    let cookie = response.cookie("id");

    // GET /login
    let response = server.get("/login").add_cookie(cookie.clone()).await;
    response.assert_status(StatusCode::SEE_OTHER);
    response.assert_header("Location", "http://localhost:5173");

    server.clear_cookies();
    server.do_not_save_cookies();

    cookie
}

#[allow(dead_code)]
pub async fn create_realm_session(
    server: &mut TestServer,
    users: Vec<User>,
) -> Vec<tower_sessions::cookie::Cookie<'static>> {
    let mut result = Vec::with_capacity(users.len());
    for user in users {
        let id = create_user_session(server, user).await;
        result.push(id);
    }
    result
}
