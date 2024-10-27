mod utils;

use axum::http::StatusCode;
use serde_json::{json, Value};
use utils::{
    containers::keycloak::User, create_basic_session, create_realm_session,
    generation::get_multipart_random_image,
};

use crate::utils::containers::keycloak::{Client, Realm};

#[tokio::test(flavor = "multi_thread")]
async fn product_test_1() {
    let realm = Realm {
        name: "product_test".to_string(),
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
            "price": 2.51,
        }))
        .add_cookie(cookies[0].clone())
        .await;

    response.assert_status(StatusCode::CREATED);
    let new_product_id = response.text();

    // GET /product with 1 product
    let response = server.get("/product").await;
    response.assert_status_ok();

    let json: Value = response.json();
    let creation_time = json.get("products").unwrap()[0]
        .get("creation_time")
        .unwrap();

    response.assert_json(&json!({
        "current_page": 0,
        "total_page": 1,
        "products": [
            {
                "id":  new_product_id,
                "name": "Bug Magnet",
                "image": image_id,
                "price": 2.51,
                "creation_time": creation_time
            }
        ]
    }));

    // GET /product/{id}
    let response = server.get(&format!("/product/{new_product_id}")).await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "id":  new_product_id,
        "name": "Bug Magnet",
        "image": image_id,
        "price": 2.51,
        "creation_time": creation_time
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
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status_ok();

    let response = server.get(&format!("/product/{new_product_id}")).await;
    response.assert_status_ok();
    response.assert_json(&json!({
        "id":  new_product_id,
        "name": "Logic Drill",
        "image": image_id,
        "price": 2.51,
        "creation_time": creation_time
    }));

    let response = server
        .put(&format!("/product/{new_product_id}"))
        .json(&json!({
            "price": 14.00,
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
        "image": image_id,
        "price": 14.00,
        "max_quantity_per_command": 2,
        "creation_time": creation_time
    }));
}

#[tokio::test(flavor = "multi_thread")]
async fn product_test_2() {
    let realm = Realm {
        name: "product_test".to_string(),
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
            "price": 2.51,
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
            "price": 1.38,
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
            "price": 3.25,
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
            "price": 0.99,
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
            "price": 4.99,
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
            "price": 2.75,
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
            "price": 1.65,
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
            "price": 3.10,
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
            "price": 2.80,
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
            "price": 1.45,
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
            "price": 2.99,
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
            "price": 4.10,
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
            "price": 3.55,
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
            "price": 5.99,
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
            "price": 3.20,
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
            "price": 2.80,
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
            "price": 1.99,
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
            "price": 4.20,
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
            "price": 2.40,
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
            "price": 3.90,
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
            "price": 5.10,
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
            "price": 1.75,
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
            "price": 2.70,
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let timeout_timer_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Concurrency Gauge",
            "image": image_id,
            "price": 3.15,
        }))
        .add_cookie(cookies[0].clone())
        .await;
    response.assert_status(StatusCode::CREATED);
    let concurrency_gauge_id = response.text();

    let response = server
        .post("/product")
        .json(&json!({
            "name": "Race Condition Stopwatch",
            "image": image_id,
            "price": 2.60,
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
            "price": 3.75,
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
            "price": 1.50,
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
            "price": 3.85,
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
            "price": 1.95,
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
                "price": 2.51,
            },
            {
                "id": logic_drill_id,
                "name": "Logic Drill",
                "image": image_id,
                "price": 1.38,
            },
            {
                "id": error_hammer_id,
                "name": "Error Hammer",
                "image": image_id,
                "price": 3.25,
            },
            {
                "id": null_pointer_detector_id,
                "name": "Null Pointer Detector",
                "image": image_id,
                "price": 0.99,
            },
            {
                "id": memory_leak_sponge_id,
                "name": "Memory Leak Sponge",
                "image": image_id,
                "price": 4.99,
            },
            {
                "id": infinite_loop_lasso_id,
                "name": "Infinite Loop Lasso",
                "image": image_id,
                "price": 2.75,
            },
            {
                "id": segfault_tape_id,
                "name": "Segmentation Fault Tape",
                "image": image_id,
                "price": 1.65,
            },
            {
                "id": four_o_four_finder_id,
                "name": "404 Finder",
                "image": image_id,
                "price": 3.10,
            },
            {
                "id": crash_cushion_id,
                "name": "Crash Cushion",
                "image": image_id,
                "price": 2.80,
            },
            {
                "id": latency_compass_id,
                "name": "Latency Compass",
                "image": image_id,
                "price": 1.45,
            },
            {
                "id": syntax_eraser_id,
                "name": "Syntax Eraser",
                "image": image_id,
                "price": 2.99,
            },
            {
                "id": thread_cutter_id,
                "name": "Concurrent Thread Cutter",
                "image": image_id,
                "price": 4.10,
            },
            {
                "id": debugger_pliers_id,
                "name": "Debugger Pliers",
                "image": image_id,
                "price": 3.55,
            },
            {
                "id": recursion_snips_id,
                "name": "Infinite Recursion Snips",
                "image": image_id,
                "price": 5.99,
            },
            {
                "id": gc_net_id,
                "name": "Garbage Collector Net",
                "image": image_id,
                "price": 3.20,
            },
            {
                "id": so_helmet_id,
                "name": "Stack Overflow Helmet",
                "image": image_id,
                "price": 2.80,
            },
            {
                "id": ruler_id,
                "name": "Off-By-One Ruler",
                "image": image_id,
                "price": 1.99,
            },
            {
                "id": deadlock_scissors_id,
                "name": "Deadlock Scissors",
                "image": image_id,
                "price": 4.20,
            },
            {
                "id": memory_dump_bag_id,
                "name": "Memory Dump Bag",
                "image": image_id,
                "price": 2.40,
            },
            {
                "id": heap_shovel_id,
                "name": "Heap Allocator Shovel",
                "image": image_id,
                "price": 3.90,
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
                "price": 5.10,
            },
            {
                "id": mutex_keychain_id,
                "name": "Mutex Lock Keychain",
                "image": image_id,
                "price": 1.75,
            },
            {
                "id": timeout_timer_id,
                "name": "Timeout Timer",
                "image": image_id,
                "price": 2.70,
            },
            {
                "id": concurrency_gauge_id,
                "name": "Concurrency Gauge",
                "image": image_id,
                "price": 3.15,
            },
            {
                "id": race_stopwatch_id,
                "name": "Race Condition Stopwatch",
                "image": image_id,
                "price": 2.60,
            },
            {
                "id": event_queue_clipboard_id,
                "name": "Event Queue Clipboard",
                "image": image_id,
                "price": 3.75,
            },
            {
                "id": uninit_pointer_bookmark_id,
                "name": "Uninitialized Pointer Bookmark",
                "image": image_id,
                "price": 1.50,
            },
            {
                "id": stack_trace_notepad_id,
                "name": "Stack Trace Notepad",
                "image": image_id,
                "price": 3.85,
            },
            {
                "id": compiler_highlighter_id,
                "name": "Compiler Warning Highlighter",
                "image": image_id,
                "price": 1.95,
            }
        ]
    }
        ));
}
