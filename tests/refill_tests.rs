mod utils;

use axum::http::StatusCode;
use serde_json::json;
use utils::{containers::keycloak::User, create_basic_session, create_realm_session};

use crate::utils::containers::keycloak::{Client, Realm};

#[test_log::test(tokio::test)]
async fn refill_test_1() {
    let realm = Realm {
        name: "refill_test".to_string(),
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

    // POST /refill new refill
    let response = server
        .post("/refill")
        .json(&json!({
            "name": "Formule Rat",
            "price": 10.0,
            "price_currency": "euro",
            "credit": 100.0,
            "credit_currency": "epicoin",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let new_refill_id = response.text();

    let response = server.get(&format!("/refill/{new_refill_id}")).await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "id":  new_refill_id,
        "name": "Formule Rat",
        "price": 10.0,
        "credit": 100.0,
        "disabled": false,
    }));

    let response = server.get("/refill").await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "current_page": 0,
        "total_page": 1,
        "refills": [
            {
                "id":  new_refill_id,
                "name": "Formule Rat",
                "price": 10.0,
                "price_currency": "euro",
                "credit": 100.0,
                "credit_currency": "epicoin",
                "disabled": false,
            }
        ]
    }));

    let response = server
        .put(&format!("/refill/{new_refill_id}"))
        .json(&json!({
            "name": "Formule Gros Rat",
            "price": 5.0,
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let response = server.get(&format!("/refill/{new_refill_id}")).await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "id":  new_refill_id,
        "name": "Formule Gros Rat",
        "price": 5.0,
        "price_currency": "euro",
        "credit": 100.0,
        "credit_currency": "epicoin",
        "disabled": false,
    }));

    let response = server.get("/refill").await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "current_page": 0,
        "total_page": 1,
        "refills": [
            {
                "id":  new_refill_id,
                "name": "Formule Gros Rat",
                "price": 5.0,
                "price_currency": "euro",
                "credit": 100.0,
                "credit_currency": "epicoin",
                "disabled": false,
            }
        ]
    }));

    let response = server
        .delete(&format!("/refill/{new_refill_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let response = server.get(&format!("/refill/{new_refill_id}")).await;
    response.assert_status_not_found();

    let response = server.get("/refill").await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "refills": []
    }));
}
