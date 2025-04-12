mod utils;

use axum::http::StatusCode;
use serde_json::json;
use utils::{containers::keycloak::User, create_basic_session, create_realm_session};

use crate::utils::containers::keycloak::{Client, Realm};

#[test_log::test(tokio::test)]
async fn location_test_1() {
    let realm = Realm {
        name: "location_test".to_string(),
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

    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // POST /location new location
    let response = server
        .post("/location")
        .json(&json!({
            "name": "Salle 401",
            "category": "room",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let new_location_id = response.text();

    let response = server.get(&format!("/location/{new_location_id}")).await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "id":  new_location_id,
        "name": "Salle 401",
        "category": "room",
        "disabled": false,
    }));

    let response = server.get("/location").await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "current_page": 0,
        "total_page": 1,
        "locations": [
            {
                "id":  new_location_id,
                "name": "Salle 401",
                "category": "room",
                "disabled": false,
            }
        ]
    }));

    let response = server
        .put(&format!("/location/{new_location_id}"))
        .json(&json!({
            "name": "Distributeur 201",
            "category": "dispenser"
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let response = server.get(&format!("/location/{new_location_id}")).await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "id":  new_location_id,
        "name": "Distributeur 201",
        "category": "dispenser",
        "disabled": false,
    }));

    let response = server.get("/location").await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "current_page": 0,
        "total_page": 1,
        "locations": [
            {
                "id":  new_location_id,
                "name": "Distributeur 201",
                "category": "dispenser",
                "disabled": false,
            }
        ]
    }));

    let response = server
        .delete(&format!("/location/{new_location_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let response = server.get(&format!("/location/{new_location_id}")).await;
    response.assert_status_not_found();

    let response = server.get("/location").await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "locations": []
    }));
}
