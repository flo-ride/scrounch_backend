mod utils;

use axum::http::StatusCode;
use serde_json::json;
use utils::{create_basic_session, create_realm_session};

use crate::utils::containers::keycloak::Realm;

#[tokio::test]
async fn warehouse_create_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // New warehouse with Ok name
    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": "Warehouse 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let id = response.text();

    let response = server
        .get(&format!("/warehouse/{id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "name": "Warehouse 1",
    }));

    let response = server
        .get("/warehouse")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "warehouses": [
            {
                "name": "Warehouse 1"
            },
        ]
    }));
}

#[tokio::test]
async fn warehouse_create_missing_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/warehouse")
        .json(&json!({}))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_unprocessable_entity();

    let response = server
        .get("/warehouse")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "warehouses": []
    }));
}

#[tokio::test]
async fn warehouse_create_empty_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": "",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server
        .get("/warehouse")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "warehouses": []
    }));
}

#[tokio::test]
async fn warehouse_create_too_long_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": "A Name Very very long, like too long, you understand ? If you don't think about it, do you want a warehouse name containing like 300 characters ? Me ... no",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server
        .get("/warehouse")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "warehouses": []
    }));
}

#[tokio::test]
async fn warehouse_create_not_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": 12,
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_unprocessable_entity();

    let response = server
        .get("/warehouse")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "warehouses": []
    }));

    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": ["1234"],
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_unprocessable_entity();

    let response = server
        .get("/warehouse")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "warehouses": []
    }));

    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": { "name": "Yes" },
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_unprocessable_entity();

    let response = server
        .get("/warehouse")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "warehouses": []
    }));
}

#[tokio::test]
async fn warehouse_edit_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // New warehouse with Ok name
    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": "Warehouse 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let id = response.text();

    let response = server
        .put(&format!("/warehouse/{id}"))
        .json(&json!({
            "name": "Warehouse 2",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let response = server
        .get(&format!("/warehouse/{id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "name": "Warehouse 2",
    }));

    let response = server
        .get("/warehouse")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "warehouses": [
            {
                "name": "Warehouse 2",
            },
        ]
    }));
}

#[tokio::test]
async fn warehouse_edit_empty_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // New warehouse with Ok name
    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": "Warehouse 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let id = response.text();

    let response = server
        .put(&format!("/warehouse/{id}"))
        .json(&json!({
            "name": "",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server
        .get(&format!("/warehouse/{id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "name": "Warehouse 1",
    }));

    let response = server
        .get("/warehouse")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "warehouses": [
            {
                "name": "Warehouse 1",
            },
        ]
    }));
}

#[tokio::test]
async fn warehouse_edit_too_long_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // New warehouse with Ok name
    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": "Warehouse 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let id = response.text();

    let response = server
        .put(&format!("/warehouse/{id}"))
        .json(&json!({
            "name": "A Name Very very long, like too long, you understand ? If you don't think about it, do you want a warehouse name containing like 300 characters ? Me ... no",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server
        .get(&format!("/warehouse/{id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "name": "Warehouse 1",
    }));

    let response = server
        .get("/warehouse")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "warehouses": [
            {
                "name": "Warehouse 1",
            },
        ]
    }));
}
