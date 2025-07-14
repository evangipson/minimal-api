use http::{request::Request, response::Response};
use http_attributes::{http_post, http_raw_post};
use std::collections::HashMap;

// ==================
// common test values
// ==================
const TEST_POST_ENDPOINT: &str = "post/test";
const TEST_POST_BODY_CONTENT: &str = "Hello!";
const TEST_POST_ID: &str = "POST-ID";

// =================
// endpoints to test
// =================
#[http_post("post/test")]
fn test_post(content: String) -> String {
    content
}

#[http_post("get/test/{id}")]
fn test_post_dynamic(content: String, id: String) -> String {
    format!("{content} {id}")
}

#[http_raw_post("post/test")]
fn test_raw_post(content: String) -> String {
    content
}

#[http_raw_post("get/test/{id}")]
fn test_raw_post_dynamic(content: String, id: String) -> String {
    format!("{content} {id}")
}

// ===============
// http_post tests
// ===============
#[test]
fn http_post_request_pattern_should_return_expected_endpoint() {
    assert_eq!(TEST_POST_ENDPOINT, test_post().request_pattern);
}

#[test]
fn http_post_handler_should_return_expected_response() {
    let expected = Response::ok(TEST_POST_BODY_CONTENT, false);
    let request = Request::new(
        TEST_POST_ENDPOINT,
        http::methods::POST,
        Some(TEST_POST_BODY_CONTENT.to_string()),
        HashMap::new(),
    );

    let result = test_post().get_response(request);

    assert_eq!(expected, result);
}

#[test]
fn http_post_handler_should_return_body_and_dynamic_path_value() {
    let expected = Response::ok(&format!("{TEST_POST_BODY_CONTENT} {TEST_POST_ID}"), false);
    let request = Request::new(
        TEST_POST_ENDPOINT,
        http::methods::POST,
        Some(TEST_POST_BODY_CONTENT.to_string()),
        HashMap::from([("id".to_string(), TEST_POST_ID.to_string())]),
    );

    let result = test_post_dynamic().get_response(request);

    assert_eq!(expected, result);
}

// ===================
// http_raw_post tests
// ===================
#[test]
fn http_raw_post_request_pattern_should_return_expected_endpoint() {
    assert_eq!(TEST_POST_ENDPOINT, test_raw_post().request_pattern);
}

#[test]
fn http_raw_post_handler_should_return_expected_raw_response() {
    let expected = Response::ok(TEST_POST_BODY_CONTENT, true);
    let request = Request::new(
        TEST_POST_ENDPOINT,
        http::methods::POST,
        Some(TEST_POST_BODY_CONTENT.to_string()),
        HashMap::new(),
    );

    let result = test_raw_post().get_response(request);

    assert_eq!(expected, result);
}

#[test]
fn http_raw_post_handler_should_return_body_and_dynamic_path_value() {
    let expected = Response::ok(&format!("{TEST_POST_BODY_CONTENT} {TEST_POST_ID}"), true);
    let request = Request::new(
        TEST_POST_ENDPOINT,
        http::methods::POST,
        Some(TEST_POST_BODY_CONTENT.to_string()),
        HashMap::from([("id".to_string(), TEST_POST_ID.to_string())]),
    );

    let result = test_raw_post_dynamic().get_response(request);

    assert_eq!(expected, result);
}
