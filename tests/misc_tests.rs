mod utils;

use crate::utils::containers::keycloak::{Client, Realm};
use utils::create_basic_session;

#[tokio::test(flavor = "multi_thread")]
async fn basic_swagger_test() {
    let realm = Realm {
        name: "misc_test".to_string(),
        clients: vec![Client::default()],
        users: vec![],
    };

    let (server, _, _nodes) = create_basic_session(realm.clone()).await;

    let response = server.get("/swagger-ui").await;
    response.assert_status_see_other();

    let response = server.get("/api-docs/openapi.json").await;
    response.assert_status_ok();
}

#[tokio::test(flavor = "multi_thread")]
async fn basic_status_test() {
    let realm = Realm {
        name: "misc_test".to_string(),
        clients: vec![Client::default()],
        users: vec![],
    };

    let (server, _, _nodes) = create_basic_session(realm.clone()).await;

    let response = server.get("/status").await;
    response.assert_status_ok();
    response.assert_text("UP");
}
