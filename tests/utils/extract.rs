#[allow(dead_code)]
pub fn extract_location_header_testresponse(response: axum_test::TestResponse) -> Option<String> {
    Some(
        response
            .headers()
            .get("Location")?
            .to_str()
            .ok()?
            .to_string(),
    )
}

#[allow(dead_code)]
pub fn extract_location_header_response(response: reqwest::Response) -> Option<String> {
    Some(
        response
            .headers()
            .get("Location")?
            .to_str()
            .ok()?
            .to_string(),
    )
}
