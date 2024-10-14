mod utils;

use axum::http::StatusCode;
use serde_json::json;
use utils::{containers::keycloak::User, create_basic_session, create_realm_session};

use crate::utils::containers::keycloak::{Client, Realm};

#[tokio::test(flavor = "multi_thread")]
async fn user_test_1() {
    let realm = Realm {
        name: "user_test".to_string(),
        clients: vec![Client::default()],
        users: vec![
            User {
                username: "user_1".to_string(),
                email: "user_1@example.com".to_string(),
                ..Default::default()
            },
            User {
                username: "user_2".to_string(),
                email: "user_2@example.com".to_string(),
                ..Default::default()
            },
            User {
                username: "user_3".to_string(),
                email: "user_3@example.com".to_string(),
                ..Default::default()
            },
        ],
    };

    let (mut server, ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server.get("/me").add_cookie(cookies[0].clone()).await;
    response.assert_status(StatusCode::OK);
    response.assert_json(
        &json!({"id": ids[0], "email": "user_1@example.com" , "username": "user_1", "name": "John Doe", "is_admin": true }),
    );

    let response = server.get("/me").add_cookie(cookies[1].clone()).await;
    response.assert_status(StatusCode::OK);
    response.assert_json(
        &json!({"id": ids[1], "email": "user_2@example.com" , "username": "user_2", "name": "John Doe", "is_admin": false }),
    );

    let response = server.get("/me").add_cookie(cookies[2].clone()).await;
    response.assert_status(StatusCode::OK);
    response.assert_json(
        &json!({"id": ids[2], "email": "user_3@example.com" , "username": "user_3", "name": "John Doe", "is_admin": false }),
    );
}
