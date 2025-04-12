mod utils;

use axum::http::StatusCode;
use serde_json::json;
use utils::{create_basic_session, create_realm_session};

use crate::utils::containers::keycloak::Realm;

#[test_log::test(tokio::test)]
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

#[test_log::test(tokio::test)]
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

#[test_log::test(tokio::test)]
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

#[test_log::test(tokio::test)]
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

#[test_log::test(tokio::test)]
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

#[test_log::test(tokio::test)]
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

#[test_log::test(tokio::test)]
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

#[test_log::test(tokio::test)]
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

#[test_log::test(tokio::test)]
async fn warehouse_create_disabled() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

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
        "disabled": false,
    }));

    let response = server
        .get("/warehouse")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "warehouses": [
            {
                "disabled": false,
            },
        ]
    }));
}

#[test_log::test(tokio::test)]
async fn warehouse_edit_disabled() {
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
            "disabled": true,
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
        "disabled": true,
    }));

    let response = server
        .get("/warehouse")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "warehouses": [
            {
                "disabled": true,
            },
        ]
    }));
}

#[test_log::test(tokio::test)]
async fn warehouse_product_create_single_product() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": "Warehouse 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let warehouse_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Product 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let product_id = response.text();

    let response = server
        .post(&format!("/warehouse/{warehouse_id}/product/{product_id}"))
        .json(&json!({
            "quantity": 10
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);

    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product/{product_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "product": {
            "name": "Product 1",
        },
    }));

    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "product": {
                    "name": "Product 1"
                }
            },
        ]
    }));
}

#[test_log::test(tokio::test)]
async fn warehouse_product_create_multiple_product() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": "Warehouse 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let warehouse_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Product 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let product_id_1 = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Product 2",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let product_id_2 = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Product 3",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let product_id_3 = response.text();

    let response = server
        .post(&format!("/warehouse/{warehouse_id}/product/{product_id_1}"))
        .json(&json!({
            "quantity": 1
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);

    let response = server
        .post(&format!("/warehouse/{warehouse_id}/product/{product_id_2}"))
        .json(&json!({
            "quantity": 1
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);

    let response = server
        .post(&format!("/warehouse/{warehouse_id}/product/{product_id_3}"))
        .json(&json!({
            "quantity": 1
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);

    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product/{product_id_1}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "product": {
            "name": "Product 1",
        },
    }));
    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product/{product_id_2}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "product": {
            "name": "Product 2",
        },
    }));
    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product/{product_id_3}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "product": {
            "name": "Product 3",
        },
    }));

    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "product": {
                    "name": "Product 1"
                }
            },
            {
                "product": {
                    "name": "Product 2"
                }
            },
            {
                "product": {
                    "name": "Product 3"
                }
            },
        ]
    }));
}

#[test_log::test(tokio::test)]
async fn warehouse_product_create_existing_link() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": "Warehouse 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let warehouse_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Product 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let product_id = response.text();

    let response = server
        .post(&format!("/warehouse/{warehouse_id}/product/{product_id}"))
        .json(&json!({
            "quantity": 10
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);

    let response = server
        .post(&format!("/warehouse/{warehouse_id}/product/{product_id}"))
        .json(&json!({
            "quantity": 10
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product/{product_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "product": {
            "name": "Product 1",
        },
    }));

    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "product": {
                    "name": "Product 1"
                }
            },
        ]
    }));
}

#[test_log::test(tokio::test)]
async fn warehouse_product_create_null_quantity() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": "Warehouse 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let warehouse_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Product 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let product_id = response.text();

    let response = server
        .post(&format!("/warehouse/{warehouse_id}/product/{product_id}"))
        .json(&json!({
            "quantity": 0
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);

    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product/{product_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "product": {
            "name": "Product 1",
        },
    }));

    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "product": {
                    "name": "Product 1"
                }
            },
        ]
    }));
}

#[test_log::test(tokio::test)]
async fn warehouse_product_create_decimal_quantity() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": "Warehouse 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let warehouse_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Product 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let product_id = response.text();

    let response = server
        .post(&format!("/warehouse/{warehouse_id}/product/{product_id}"))
        .json(&json!({
            "quantity": 0.1
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);

    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product/{product_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "product": {
            "name": "Product 1",
        },
    }));

    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "product": {
                    "name": "Product 1"
                }
            },
        ]
    }));
}

#[test_log::test(tokio::test)]
async fn warehouse_product_create_negative_quantity() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/warehouse")
        .json(&json!({
            "name": "Warehouse 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let warehouse_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Product 1",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let product_id = response.text();

    let response = server
        .post(&format!("/warehouse/{warehouse_id}/product/{product_id}"))
        .json(&json!({
            "quantity": -1
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server
        .get(&format!("/warehouse/{warehouse_id}/product"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "products": [],
        "total_page": 1,
        "current_page": 0,
    }));
}
