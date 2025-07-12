use http::{request::Request, response::Response};
use http_attributes::{http_put, http_raw_put};
use std::collections::HashMap;

// ==================
// common test values
// ==================
const TEST_PUT_ENDPOINT: &str = "put/test";
const TEST_PUT_BODY_CONTENT: &str = "Hello!";
const TEST_PUT_ID: &str = "PUT-ID";

// =================
// endpoints to test
// =================
#[http_put("put/test")]
fn test_put(content: String) -> String {
    content
}

#[http_put("put/test/{id}")]
fn test_put_dynamic(content: String, id: String) -> String {
    format!("{content} {id}")
}

#[http_raw_put("put/test")]
fn test_raw_put(content: String) -> String {
    content
}

#[http_raw_put("put/test/{id}")]
fn test_raw_put_dynamic(content: String, id: String) -> String {
    format!("{content} {id}")
}

// ==============
// http_put tests
// ==============
#[test]
fn http_put_request_pattern_should_return_expected_endpoint() {
    assert_eq!(TEST_PUT_ENDPOINT, test_put().request_pattern);
}

#[test]
fn http_put_handler_should_return_expected_request() {
    let expected = Response::ok(TEST_PUT_BODY_CONTENT, false);
    let request = Request::new(
        TEST_PUT_ENDPOINT,
        http::methods::PUT,
        Some(TEST_PUT_BODY_CONTENT.to_string()),
        HashMap::new(),
    );

    let result = (test_put().handler)(request);

    assert_eq!(expected, result);
}

#[test]
fn http_put_handler_should_return_dynamic_route_value() {
    let expected = Response::ok(&format!("{TEST_PUT_BODY_CONTENT} {TEST_PUT_ID}"), false);
    let request = Request::new(
        TEST_PUT_ENDPOINT,
        http::methods::PUT,
        Some(TEST_PUT_BODY_CONTENT.to_string()),
        HashMap::from([("id".to_string(), TEST_PUT_ID.to_string())]),
    );

    let result = (test_put_dynamic().handler)(request);

    assert_eq!(expected, result);
}

// ==================
// http_raw_put tests
// ==================
#[test]
fn http_raw_put_request_pattern_should_return_expected_endpoint() {
    assert_eq!(TEST_PUT_ENDPOINT, test_raw_put().request_pattern);
}

#[test]
fn http_raw_put_handler_should_return_expected_raw_response() {
    let expected = Response::ok(TEST_PUT_BODY_CONTENT, true);
    let request = Request::new(
        TEST_PUT_ENDPOINT,
        http::methods::PUT,
        Some(TEST_PUT_BODY_CONTENT.to_string()),
        HashMap::new(),
    );

    let result = (test_raw_put().handler)(request);

    assert_eq!(expected, result);
}

#[test]
fn http_raw_put_handler_should_return_dynamic_route_value() {
    let expected = Response::ok(&format!("{TEST_PUT_BODY_CONTENT} {TEST_PUT_ID}"), true);
    let request = Request::new(
        TEST_PUT_ENDPOINT,
        http::methods::PUT,
        Some(TEST_PUT_BODY_CONTENT.to_string()),
        HashMap::from([("id".to_string(), TEST_PUT_ID.to_string())]),
    );

    let result = (test_raw_put_dynamic().handler)(request);

    assert_eq!(expected, result);
}
