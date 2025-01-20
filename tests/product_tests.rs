mod utils;

use axum::http::StatusCode;
use serde_json::{json, Value};
use utils::{create_basic_session, create_realm_session, generation::get_multipart_random_image};

use crate::utils::containers::keycloak::Realm;

#[tokio::test]
async fn product_test_1() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // GET /product with not product
    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json(&json!({ "total_page": 1, "current_page": 0, "products": [] }));

    // GET /product/id doesn't exist
    let response = server
        .get("/product/1a731f58-18f1-4c95-8de5-611bde07f4f1")
        .await;
    response.assert_status_not_found();

    // POST /upload new image
    let response = server
        .post("/upload")
        .multipart(get_multipart_random_image("bug_magnet", "random_name").await)
        .add_query_param("type", "product")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    let json: Vec<(String, String)> = response.json();
    assert_eq!(json[0].0, "bug_magnet.jpeg");
    let image_id = json[0].1.clone();

    // POST /product new product
    let response = server
        .post("/product")
        .json(&json!({
            "name": "Bug Magnet",
            "image": image_id,
            "sell_price": 2.51,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;

    response.assert_status(StatusCode::CREATED);
    let new_product_id = response.text();

    // GET /product with 1 product
    let response = server.get("/product").await;
    response.assert_status_ok();

    let json: Value = response.json();
    let created_at = json.get("products").unwrap()[0].get("created_at").unwrap();

    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "products": [
            {
                "id":  new_product_id,
                "name": "Bug Magnet",
                "display_order": 0,
                "image": image_id,
                "sell_price": 2.51,
                "sell_price_currency": "euro",
                "unit": "unit",
                "purchasable": true,
                "created_at": created_at,
                "disabled": false,
            }
        ]
    }));

    // GET /product/{id}
    let response = server.get(&format!("/product/{new_product_id}")).await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "id":  new_product_id,
        "name": "Bug Magnet",
        "display_order": 0,
        "image": image_id,
        "sell_price": 2.51,
        "sell_price_currency": "euro",
        "unit": "unit",
        "purchasable": true,
        "created_at": created_at,
        "disabled": false,
    }));

    let response = server
        .get(&format!("/download/{image_id}"))
        .add_query_param("type", "product")
        .await;
    response.assert_status_ok();

    let response = server
        .put(&format!("/product/{new_product_id}"))
        .json(&json!({
            "name": "Logic Drill",
            "unit": "meter",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let response = server.get(&format!("/product/{new_product_id}")).await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "id":  new_product_id,
        "name": "Logic Drill",
        "display_order": 0,
        "image": image_id,
        "sell_price": 2.51,
        "sell_price_currency": "euro",
        "unit": "meter",
        "purchasable": true,
        "created_at": created_at,
        "disabled": false,
    }));

    let response = server
        .put(&format!("/product/{new_product_id}"))
        .json(&json!({
            "sell_price": 14.00,
            "max_quantity_per_command": 2,
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let response = server.get(&format!("/product/{new_product_id}")).await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "id":  new_product_id,
        "name": "Logic Drill",
        "display_order": 0,
        "image": image_id,
        "sell_price": 14.00,
        "sell_price_currency": "euro",
        "max_quantity_per_command": 2,
        "unit": "meter",
        "purchasable": true,
        "created_at": created_at,
        "disabled": false,
    }));

    let response = server
        .delete(&format!("/product/{new_product_id}"))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let response = server.get(&format!("/product/{new_product_id}")).await;
    response.assert_status_not_found();
}

#[tokio::test]
async fn product_test_2() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // POST /upload new image
    let response = server
        .post("/upload")
        .multipart(get_multipart_random_image("bug_magnet", "random_name").await)
        .add_query_param("type", "product")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    let json: Vec<(String, String)> = response.json();
    assert_eq!(json[0].0, "bug_magnet.jpeg");
    let image_id = json[0].1.clone();

    // POST /product new product
    let response = server
        .post("/product")
        .json(&json!({
            "name": "Bug Magnet",
            "image": image_id,
            "sell_price": 2.51,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let bug_magnet_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Logic Drill",
            "image": image_id,
            "sell_price": 1.38,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let logic_drill_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Error Hammer",
            "image": image_id,
            "sell_price": 3.25,
            "sell_price_currency": "epicoin",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let error_hammer_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Null Pointer Detector",
            "image": image_id,
            "sell_price": 0.99,
            "sell_price_currency": "epicoin",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let null_pointer_detector_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Memory Leak Sponge",
            "image": image_id,
            "sell_price": 4.99,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let memory_leak_sponge_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Infinite Loop Lasso",
            "image": image_id,
            "sell_price": 2.75,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let infinite_loop_lasso_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Segmentation Fault Tape",
            "image": image_id,
            "sell_price": 1.65,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let segfault_tape_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "404 Finder",
            "image": image_id,
            "sell_price": 3.10,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let four_o_four_finder_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Crash Cushion",
            "image": image_id,
            "sell_price": 2.80,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let crash_cushion_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Latency Compass",
            "image": image_id,
            "sell_price": 1.45,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let latency_compass_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Syntax Eraser",
            "image": image_id,
            "sell_price": 2.99,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let syntax_eraser_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Concurrent Thread Cutter",
            "image": image_id,
            "sell_price": 4.10,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let thread_cutter_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Debugger Pliers",
            "image": image_id,
            "sell_price": 3.55,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let debugger_pliers_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Infinite Recursion Snips",
            "image": image_id,
            "sell_price": 5.99,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let recursion_snips_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Garbage Collector Net",
            "image": image_id,
            "sell_price": 3.20,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let gc_net_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Stack Overflow Helmet",
            "image": image_id,
            "sell_price": 2.80,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let so_helmet_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Off-By-One Ruler",
            "image": image_id,
            "sell_price": 1.99,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let ruler_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Deadlock Scissors",
            "image": image_id,
            "sell_price": 4.20,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let deadlock_scissors_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Memory Dump Bag",
            "image": image_id,
            "sell_price": 2.40,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let memory_dump_bag_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Heap Allocator Shovel",
            "image": image_id,
            "sell_price": 3.90,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let heap_shovel_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Semaphore Semaphore",
            "image": image_id,
            "sell_price": 5.10,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let semaphore_semaphore_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Mutex Lock Keychain",
            "image": image_id,
            "sell_price": 1.75,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let mutex_keychain_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Timeout Timer",
            "image": image_id,
            "sell_price": 2.70,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let timeout_timer_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Consell_price_currency Gauge",
            "image": image_id,
            "sell_price": 3.15,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let consell_price_currency_gauge_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Race Condition Stopwatch",
            "image": image_id,
            "sell_price": 2.60,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let race_stopwatch_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Event Queue Clipboard",
            "image": image_id,
            "sell_price": 3.75,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let event_queue_clipboard_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Uninitialized Pointer Bookmark",
            "image": image_id,
            "sell_price": 1.50,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let uninit_pointer_bookmark_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Stack Trace Notepad",
            "image": image_id,
            "sell_price": 3.85,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let stack_trace_notepad_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Compiler Warning Highlighter",
            "image": image_id,
            "sell_price": 1.95,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let compiler_highlighter_id = response.text();

    // GET /product
    let response = server.get("/product").await;
    response.assert_status_ok();

    response.assert_json_contains(&json!({
        "current_page": 0,
        "total_page": 2,
        "products": [
            {
                "id": bug_magnet_id,
                "name": "Bug Magnet",
                "image": image_id,
                "sell_price": 2.51,
                "sell_price_currency": "euro",
            },
            {
                "id": logic_drill_id,
                "name": "Logic Drill",
                "image": image_id,
                "sell_price": 1.38,
                "sell_price_currency": "euro",
            },
            {
                "id": error_hammer_id,
                "name": "Error Hammer",
                "image": image_id,
                "sell_price": 3.25,
                "sell_price_currency": "epicoin",
            },
            {
                "id": null_pointer_detector_id,
                "name": "Null Pointer Detector",
                "image": image_id,
                "sell_price": 0.99,
                "sell_price_currency": "epicoin",
            },
            {
                "id": memory_leak_sponge_id,
                "name": "Memory Leak Sponge",
                "image": image_id,
                "sell_price": 4.99,
                "sell_price_currency": "euro",
            },
            {
                "id": infinite_loop_lasso_id,
                "name": "Infinite Loop Lasso",
                "image": image_id,
                "sell_price": 2.75,
                "sell_price_currency": "euro",
            },
            {
                "id": segfault_tape_id,
                "name": "Segmentation Fault Tape",
                "image": image_id,
                "sell_price": 1.65,
                "sell_price_currency": "euro",
            },
            {
                "id": four_o_four_finder_id,
                "name": "404 Finder",
                "image": image_id,
                "sell_price": 3.10,
                "sell_price_currency": "euro",
            },
            {
                "id": crash_cushion_id,
                "name": "Crash Cushion",
                "image": image_id,
                "sell_price": 2.80,
                "sell_price_currency": "euro",
            },
            {
                "id": latency_compass_id,
                "name": "Latency Compass",
                "image": image_id,
                "sell_price": 1.45,
                "sell_price_currency": "euro",
            },
            {
                "id": syntax_eraser_id,
                "name": "Syntax Eraser",
                "image": image_id,
                "sell_price": 2.99,
                "sell_price_currency": "euro",
            },
            {
                "id": thread_cutter_id,
                "name": "Concurrent Thread Cutter",
                "image": image_id,
                "sell_price": 4.10,
                "sell_price_currency": "euro",
            },
            {
                "id": debugger_pliers_id,
                "name": "Debugger Pliers",
                "image": image_id,
                "sell_price": 3.55,
                "sell_price_currency": "euro",
            },
            {
                "id": recursion_snips_id,
                "name": "Infinite Recursion Snips",
                "image": image_id,
                "sell_price": 5.99,
                "sell_price_currency": "euro",
            },
            {
                "id": gc_net_id,
                "name": "Garbage Collector Net",
                "image": image_id,
                "sell_price": 3.20,
                "sell_price_currency": "euro",
            },
            {
                "id": so_helmet_id,
                "name": "Stack Overflow Helmet",
                "image": image_id,
                "sell_price": 2.80,
                "sell_price_currency": "euro",
            },
            {
                "id": ruler_id,
                "name": "Off-By-One Ruler",
                "image": image_id,
                "sell_price": 1.99,
                "sell_price_currency": "euro",
            },
            {
                "id": deadlock_scissors_id,
                "name": "Deadlock Scissors",
                "image": image_id,
                "sell_price": 4.20,
                "sell_price_currency": "euro",
            },
            {
                "id": memory_dump_bag_id,
                "name": "Memory Dump Bag",
                "image": image_id,
                "sell_price": 2.40,
                "sell_price_currency": "euro",
            },
            {
                "id": heap_shovel_id,
                "name": "Heap Allocator Shovel",
                "image": image_id,
                "sell_price": 3.90,
                "sell_price_currency": "euro",
            },
        ]
    }));

    let response = server.get("/product").add_query_param("page", "1").await;
    response.assert_status_ok();

    response.assert_json_contains(&json!({
        "current_page": 1,
        "total_page": 2,
        "products": [
            {
                "id": semaphore_semaphore_id,
                "name": "Semaphore Semaphore",
                "image": image_id,
                "sell_price": 5.10,
                "sell_price_currency": "euro",
            },
            {
                "id": mutex_keychain_id,
                "name": "Mutex Lock Keychain",
                "image": image_id,
                "sell_price": 1.75,
                "sell_price_currency": "euro",
            },
            {
                "id": timeout_timer_id,
                "name": "Timeout Timer",
                "image": image_id,
                "sell_price": 2.70,
                "sell_price_currency": "euro",
            },
            {
                "id": consell_price_currency_gauge_id,
                "name": "Consell_price_currency Gauge",
                "image": image_id,
                "sell_price": 3.15,
                "sell_price_currency": "euro",
            },
            {
                "id": race_stopwatch_id,
                "name": "Race Condition Stopwatch",
                "image": image_id,
                "sell_price": 2.60,
                "sell_price_currency": "euro",
            },
            {
                "id": event_queue_clipboard_id,
                "name": "Event Queue Clipboard",
                "image": image_id,
                "sell_price": 3.75,
                "sell_price_currency": "euro",
            },
            {
                "id": uninit_pointer_bookmark_id,
                "name": "Uninitialized Pointer Bookmark",
                "image": image_id,
                "sell_price": 1.50,
                "sell_price_currency": "euro",
            },
            {
                "id": stack_trace_notepad_id,
                "name": "Stack Trace Notepad",
                "image": image_id,
                "sell_price": 3.85,
                "sell_price_currency": "euro",
            },
            {
                "id": compiler_highlighter_id,
                "name": "Compiler Warning Highlighter",
                "image": image_id,
                "sell_price": 1.95,
                "sell_price_currency": "euro",
            }
        ]
    }
        ));
}

#[tokio::test]
async fn product_create_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // New product with Ok name
    let response = server
        .post("/product")
        .json(&json!({
            "name": "Bug Magnet",
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let id = response.text();

    let response = server.get(&format!("/product/{id}")).await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "name": "Bug Magnet",
    }));

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "name": "Bug Magnet"
            },
        ]
    }));
}

#[tokio::test]
async fn product_create_missing_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/product")
        .json(&json!({
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "products": []
    }));
}

#[tokio::test]
async fn product_create_empty_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/product")
        .json(&json!({
            "name": "",
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "products": []
    }));
}

#[tokio::test]
async fn product_create_too_long_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/product")
        .json(&json!({
            "name": "A Name Very very long, like too long, you understand ? If you don't think about it, do you want a product name containing like 300 characters ? Me ... no",
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "products": []
    }));
}

#[tokio::test]
async fn product_create_not_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/product")
        .json(&json!({
            "name": 12,
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "products": []
    }));

    let response = server
        .post("/product")
        .json(&json!({
            "name": ["1234"],
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "products": []
    }));

    let response = server
        .post("/product")
        .json(&json!({
            "name": { "name": "Yes" },
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "products": []
    }));
}

#[tokio::test]
async fn product_edit_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // New product with Ok name
    let response = server
        .post("/product")
        .json(&json!({
            "name": "Bug Magnet",
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let id = response.text();

    let response = server
        .put(&format!("/product/{id}"))
        .json(&json!({
            "name": "Buggy Magnet",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let response = server.get(&format!("/product/{id}")).await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "name": "Buggy Magnet",
    }));

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "name": "Buggy Magnet",
            },
        ]
    }));
}

#[tokio::test]
async fn product_edit_empty_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // New product with Ok name
    let response = server
        .post("/product")
        .json(&json!({
            "name": "Bug Magnet",
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let id = response.text();

    let response = server
        .put(&format!("/product/{id}"))
        .json(&json!({
            "name": "",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server.get(&format!("/product/{id}")).await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "name": "Bug Magnet",
    }));

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "name": "Bug Magnet",
            },
        ]
    }));
}

#[tokio::test]
async fn product_edit_too_long_name() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // New product with Ok name
    let response = server
        .post("/product")
        .json(&json!({
            "name": "Bug Magnet",
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let id = response.text();

    let response = server
        .put(&format!("/product/{id}"))
        .json(&json!({
            "name": "A Name Very very long, like too long, you understand ? If you don't think about it, do you want a product name containing like 300 characters ? Me ... no",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server.get(&format!("/product/{id}")).await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "name": "Bug Magnet",
    }));

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "name": "Bug Magnet",
            },
        ]
    }));
}

#[tokio::test]
async fn product_create_image() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/upload")
        .multipart(get_multipart_random_image("bug_magnet", "random_name").await)
        .add_query_param("type", "product")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    let json: Vec<(String, String)> = response.json();
    assert_eq!(json[0].0, "bug_magnet.jpeg");
    let image_id = json[0].1.clone();

    // New product with Ok name
    let response = server
        .post("/product")
        .json(&json!({
            "image": image_id,
            "name": "Bug Magnet",
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let id = response.text();

    let response = server.get(&format!("/product/{id}")).await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "image": image_id,
    }));

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "image": image_id,
            },
        ]
    }));

    let response = server
        .get(&format!("/download/{image_id}"))
        .add_query_param("type", "product")
        .await;
    response.assert_status_ok();
}

#[tokio::test]
async fn product_create_image_doesnt_exit() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // New product with Ok name
    let response = server
        .post("/product")
        .json(&json!({
            "image": "00000-91994943-13929301-94919.jpg", // Some fake id
            "name": "Bug Magnet",
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request()
}

#[tokio::test]
async fn product_edit_image() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/upload")
        .multipart(get_multipart_random_image("bug_magnet", "random_name").await)
        .add_query_param("type", "product")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    let json: Vec<(String, String)> = response.json();
    assert_eq!(json[0].0, "bug_magnet.jpeg");
    let image_id = json[0].1.clone();

    // New product with Ok name
    let response = server
        .post("/product")
        .json(&json!({
            "image": image_id,
            "name": "Bug Magnet",
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let id = response.text();

    let response = server
        .post("/upload")
        .multipart(get_multipart_random_image("bug_magnet_2", "random_name_2").await)
        .add_query_param("type", "product")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    let json: Vec<(String, String)> = response.json();
    assert_eq!(json[0].0, "bug_magnet_2.jpeg");
    let new_image_id = json[0].1.clone();

    let response = server
        .put(&format!("/product/{id}"))
        .json(&json!({
            "image": new_image_id
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let response = server.get(&format!("/product/{id}")).await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "image": new_image_id,
    }));

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "image": new_image_id,
            },
        ]
    }));

    let response = server
        .get(&format!("/download/{image_id}"))
        .add_query_param("type", "product")
        .await;
    response.assert_status_not_found();

    let response = server
        .get(&format!("/download/{new_image_id}"))
        .add_query_param("type", "product")
        .await;
    response.assert_status_ok();
}

#[tokio::test]
async fn product_edit_image_doesnt_exist() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    let response = server
        .post("/upload")
        .multipart(get_multipart_random_image("bug_magnet", "random_name").await)
        .add_query_param("type", "product")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();
    let json: Vec<(String, String)> = response.json();
    assert_eq!(json[0].0, "bug_magnet.jpeg");
    let image_id = json[0].1.clone();

    // New product with Ok name
    let response = server
        .post("/product")
        .json(&json!({
            "image": image_id,
            "name": "Bug Magnet",
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let id = response.text();

    let response = server
        .put(&format!("/product/{id}"))
        .json(&json!({
            "image": "00000-91994943-94919.jpg",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_bad_request();

    let response = server.get(&format!("/product/{id}")).await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "image": image_id,
    }));

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "image": image_id,
            },
        ]
    }));

    // The previous image is not deleted for now
    let response = server
        .get(&format!("/download/{image_id}"))
        .add_query_param("type", "product")
        .await;
    response.assert_status_ok();
}

#[tokio::test]
async fn product_append_image_after_creation() {
    let realm = Realm::default();
    let (mut server, _ids, _nodes) = create_basic_session(realm.clone()).await;
    let cookies = create_realm_session(&mut server, realm.users).await;

    // New product with Ok name
    let response = server
        .post("/product")
        .json(&json!({
            "name": "Bug Magnet",
            "sell_price": 0.01,
            "sell_price_currency": "euro",
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let id = response.text();

    let response = server
        .post("/upload")
        .multipart(get_multipart_random_image("bug_magnet", "random_name").await)
        .add_query_param("type", "product")
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let json: Vec<(String, String)> = response.json();
    assert_eq!(json[0].0, "bug_magnet.jpeg");
    let image_id = json[0].1.clone();

    let response = server
        .put(&format!("/product/{id}"))
        .json(&json!({
            "image": image_id
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let response = server.get(&format!("/product/{id}")).await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "image": image_id,
    }));

    let response = server.get("/product").await;
    response.assert_status_ok();
    response.assert_json_contains(&json!({
        "products": [
            {
                "image": image_id,
            },
        ]
    }));

    let response = server
        .get(&format!("/download/{image_id}"))
        .add_query_param("type", "product")
        .await;
    response.assert_status_ok();
}
