mod utils;

use axum::http::StatusCode;
use serde_json::json;
use utils::{create_basic_session, create_realm_session};

use crate::utils::containers::keycloak::Realm;

#[tokio::test]
async fn recipe_create_zero_ingredient() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // Create each step
    let cake_id = server
        .post("/product")
        .json(&json!({
            "name": "Cake",
            "sell_price": 1.00,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let response = server
        .post("/recipe")
        .json(&json!({
            "name": "Recipe for a Cake",
            "product": cake_id,
            "ingredients": []

        }))
        .add_cookie(cookies[0].clone())
        .await;

    response.assert_status(StatusCode::CREATED);
    let recipe_id = response.text();

    let response = server.get("/recipe").add_cookie(cookies[0].clone()).await;
    response.assert_status_ok();

    response.assert_json_contains(&json!({
        "current_page": 0,
        "total_page": 1,
        "recipes": [
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [],
                "disabled": false,
            }
        ]
    }));

    let response = server
        .get(&format!("/recipe/{recipe_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    response.assert_json_contains(&json!(
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [],
                "disabled": false,
            }
    ));
}

#[tokio::test]
async fn recipe_create_one_ingredient() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // Create each step
    let cake_id = server
        .post("/product")
        .json(&json!({
            "name": "Cake",
            "sell_price": 1.00,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    // Create each step
    let eggs_id = server
        .post("/product")
        .json(&json!({
            "name": "Eggs",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let response = server
        .post("/recipe")
        .json(&json!({
            "name": "Recipe for a Cake",
            "product": cake_id,
            "ingredients": [
                { "product": eggs_id, "quantity": 3 },
            ]

        }))
        .add_cookie(cookies[0].clone())
        .await;

    response.assert_status(StatusCode::CREATED);
    let recipe_id = response.text();

    let response = server.get("/recipe").add_cookie(cookies[0].clone()).await;
    response.assert_status_ok();

    response.assert_json_contains(&json!({
        "current_page": 0,
        "total_page": 1,
        "recipes": [
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [
                    { "product": eggs_id, "quantity": 3.0, "disabled": false },
                ],
                "disabled": false,
            }
        ]
    }));

    let response = server
        .get(&format!("/recipe/{recipe_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    response.assert_json_contains(&json!(
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [
                    { "product": eggs_id, "quantity": 3.0, "disabled": false },
                ],
                "disabled": false,
            }
    ));
}

#[tokio::test]
async fn recipe_create_multiple_ingredients() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // Create each step
    let cake_id = server
        .post("/product")
        .json(&json!({
            "name": "Cake",
            "sell_price": 1.00,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    // Create each step
    let eggs_id = server
        .post("/product")
        .json(&json!({
            "name": "Eggs",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let milk_id = server
        .post("/product")
        .json(&json!({
            "name": "Milk",
            "unit": "liter",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let strawberry_id = server
        .post("/product")
        .json(&json!({
            "name": "Strawberry",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let response = server
        .post("/recipe")
        .json(&json!({
            "name": "Recipe for a Cake",
            "product": cake_id,
            "ingredients": [
                { "product": eggs_id, "quantity": 3 },
                { "product": milk_id, "quantity": 0.5 },
                { "product": strawberry_id, "quantity": 25 },
            ]

        }))
        .add_cookie(cookies[0].clone())
        .await;

    response.assert_status(StatusCode::CREATED);
    let recipe_id = response.text();

    let response = server.get("/recipe").add_cookie(cookies[0].clone()).await;
    response.assert_status_ok();

    response.assert_json_contains(&json!({
        "current_page": 0,
        "total_page": 1,
        "recipes": [
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [
                    { "product": milk_id, "quantity": 0.5, "disabled": false },
                    { "product": eggs_id, "quantity": 3.0, "disabled": false },
                    { "product": strawberry_id, "quantity": 25.0, "disabled": false },
                ],
                "disabled": false,
            }
        ]
    }));

    let response = server
        .get(&format!("/recipe/{recipe_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    response.assert_json_contains(&json!(
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [
                    { "product": milk_id, "quantity": 0.5, "disabled": false },
                    { "product": eggs_id, "quantity": 3.0, "disabled": false },
                    { "product": strawberry_id, "quantity": 25.0, "disabled": false },
                ],
                "disabled": false,
            }
    ));
}

#[tokio::test]
async fn recipe_edit_add_ingredient() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // Create each step
    let cake_id = server
        .post("/product")
        .json(&json!({
            "name": "Cake",
            "sell_price": 1.00,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    // Create each step
    let eggs_id = server
        .post("/product")
        .json(&json!({
            "name": "Eggs",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let milk_id = server
        .post("/product")
        .json(&json!({
            "name": "Milk",
            "unit": "liter",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let strawberry_id = server
        .post("/product")
        .json(&json!({
            "name": "Strawberry",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let recipe_id = server
        .post("/recipe")
        .json(&json!({
            "name": "Recipe for a Cake",
            "product": cake_id,
            "ingredients": [
                { "product": milk_id, "quantity": 0.5 },
                { "product": eggs_id, "quantity": 3 },
            ]

        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let response = server
        .put(&format!("/recipe/{recipe_id}"))
        .json(&json!({
            "ingredients": [
                { "product": milk_id, "quantity": 0.5 },
                { "product": eggs_id, "quantity": 3 },
                { "product": strawberry_id, "quantity": 25 },
            ]
        }))
        .add_cookie(cookies[0].clone())
        .await;

    response.assert_status_ok();

    let response = server.get("/recipe").add_cookie(cookies[0].clone()).await;
    response.assert_status_ok();

    response.assert_json_contains(&json!({
        "current_page": 0,
        "total_page": 1,
        "recipes": [
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [
                    { "product": milk_id, "quantity": 0.5, "disabled": false },
                    { "product": eggs_id, "quantity": 3.0, "disabled": false },
                    { "product": strawberry_id, "quantity": 25.0, "disabled": false },
                ],
                "disabled": false,
            }
        ]
    }));

    let response = server
        .get(&format!("/recipe/{recipe_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    response.assert_json_contains(&json!(
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [
                    { "product": milk_id, "quantity": 0.5, "disabled": false },
                    { "product": eggs_id, "quantity": 3.0, "disabled": false },
                    { "product": strawberry_id, "quantity": 25.0, "disabled": false },
                ],
                "disabled": false,
            }
    ));
}

#[tokio::test]
async fn recipe_edit_remove_ingredient() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // Create each step
    let cake_id = server
        .post("/product")
        .json(&json!({
            "name": "Cake",
            "sell_price": 1.00,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    // Create each step
    let eggs_id = server
        .post("/product")
        .json(&json!({
            "name": "Eggs",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let milk_id = server
        .post("/product")
        .json(&json!({
            "name": "Milk",
            "unit": "liter",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let strawberry_id = server
        .post("/product")
        .json(&json!({
            "name": "Strawberry",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let recipe_id = server
        .post("/recipe")
        .json(&json!({
            "name": "Recipe for a Cake",
            "product": cake_id,
            "ingredients": [
                { "product": eggs_id, "quantity": 3 },
                { "product": milk_id, "quantity": 0.5 },
                { "product": strawberry_id, "quantity": 25 },
            ]

        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let response = server
        .put(&format!("/recipe/{recipe_id}"))
        .json(&json!({
            "ingredients": [
                { "product": eggs_id, "quantity": 3 },
                { "product": strawberry_id, "quantity": 25 },
            ]
        }))
        .add_cookie(cookies[0].clone())
        .await;

    response.assert_status_ok();

    let response = server.get("/recipe").add_cookie(cookies[0].clone()).await;
    response.assert_status_ok();

    response.assert_json_contains(&json!({
        "current_page": 0,
        "total_page": 1,
        "recipes": [
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [
                    { "product": eggs_id, "quantity": 3.0, "disabled": false },
                    { "product": strawberry_id, "quantity": 25.0, "disabled": false },
                ],
                "disabled": false,
            }
        ]
    }));

    let response = server
        .get(&format!("/recipe/{recipe_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    response.assert_json_contains(&json!(
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [
                    { "product": eggs_id, "quantity": 3.0, "disabled": false },
                    { "product": strawberry_id, "quantity": 25.0, "disabled": false },
                ],
                "disabled": false,
            }
    ));
}

#[tokio::test]
async fn recipe_edit_add_remove_ingredients() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // Create each step
    let cake_id = server
        .post("/product")
        .json(&json!({
            "name": "Cake",
            "sell_price": 1.00,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    // Create each step
    let eggs_id = server
        .post("/product")
        .json(&json!({
            "name": "Eggs",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let milk_id = server
        .post("/product")
        .json(&json!({
            "name": "Milk",
            "unit": "liter",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let strawberry_id = server
        .post("/product")
        .json(&json!({
            "name": "Strawberry",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let recipe_id = server
        .post("/recipe")
        .json(&json!({
            "name": "Recipe for a Cake",
            "product": cake_id,
            "ingredients": [
                { "product": eggs_id, "quantity": 3 },
                { "product": milk_id, "quantity": 0.5 },
            ]

        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let response = server
        .put(&format!("/recipe/{recipe_id}"))
        .json(&json!({
            "ingredients": [
                { "product": eggs_id, "quantity": 3 },
                { "product": strawberry_id, "quantity": 25 },
            ]
        }))
        .add_cookie(cookies[0].clone())
        .await;

    response.assert_status_ok();

    let response = server.get("/recipe").add_cookie(cookies[0].clone()).await;
    response.assert_status_ok();

    response.assert_json_contains(&json!({
        "current_page": 0,
        "total_page": 1,
        "recipes": [
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [
                    { "product": eggs_id, "quantity": 3.0, "disabled": false },
                    { "product": strawberry_id, "quantity": 25.0, "disabled": false },
                ],
                "disabled": false,
            }
        ]
    }));

    let response = server
        .get(&format!("/recipe/{recipe_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    response.assert_json_contains(&json!(
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [
                    { "product": eggs_id, "quantity": 3.0, "disabled": false },
                    { "product": strawberry_id, "quantity": 25.0, "disabled": false },
                ],
                "disabled": false,
            }
    ));
}

#[tokio::test]
async fn recipe_edit_quantity() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // Create each step
    let cake_id = server
        .post("/product")
        .json(&json!({
            "name": "Cake",
            "sell_price": 1.00,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    // Create each step
    let eggs_id = server
        .post("/product")
        .json(&json!({
            "name": "Eggs",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let milk_id = server
        .post("/product")
        .json(&json!({
            "name": "Milk",
            "unit": "liter",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let strawberry_id = server
        .post("/product")
        .json(&json!({
            "name": "Strawberry",
            "purchasable": false,
        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let recipe_id = server
        .post("/recipe")
        .json(&json!({
            "name": "Recipe for a Cake",
            "product": cake_id,
            "ingredients": [
                { "product": eggs_id, "quantity": 3 },
                { "product": milk_id, "quantity": 0.5 },
                { "product": strawberry_id, "quantity": 25 },
            ]

        }))
        .add_cookie(cookies[0].clone())
        .await
        .text();

    let response = server
        .put(&format!("/recipe/{recipe_id}"))
        .json(&json!({
            "ingredients": [
                { "product": eggs_id, "quantity": 6 },
                { "product": milk_id, "quantity": 0.5 },
                { "product": strawberry_id, "quantity": 25 },
            ]
        }))
        .add_cookie(cookies[0].clone())
        .await;

    response.assert_status_ok();

    let response = server.get("/recipe").add_cookie(cookies[0].clone()).await;
    response.assert_status_ok();

    response.assert_json_contains(&json!({
        "current_page": 0,
        "total_page": 1,
        "recipes": [
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [
                    { "product": milk_id, "quantity": 0.5, "disabled": false },
                    { "product": eggs_id, "quantity": 6.0, "disabled": false },
                    { "product": strawberry_id, "quantity": 25.0, "disabled": false },
                ],
                "disabled": false,
            }
        ]
    }));

    let response = server
        .get(&format!("/recipe/{recipe_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    response.assert_json_contains(&json!(
            {
                "id":  recipe_id,
                "name": "Recipe for a Cake",
                "product": cake_id,
                "ingredients": [
                    { "product": milk_id, "quantity": 0.5, "disabled": false },
                    { "product": eggs_id, "quantity": 6.0, "disabled": false },
                    { "product": strawberry_id, "quantity": 25.0, "disabled": false },
                ],
                "disabled": false,
            }
    ));
}
