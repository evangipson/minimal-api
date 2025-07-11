use http::{request::Request, response::Response};
use http_attributes::{http_get, http_raw_get};
use std::collections::HashMap;

// ==================
// common test values
// ==================
const TEST_GET_ENDPOINT: &str = "get/test";
const TEST_GET_MESSAGE: &str = "Hello!";
const TEST_GET_QUERY_ENDPOINT: &str = "get/test/query";
const TEST_GET_ID: &str = "TEST-GET";

// =================
// endpoints to test
// =================
#[http_get("get/test")]
fn test_get() -> String {
    TEST_GET_MESSAGE.to_string()
}

#[http_get("get/test/query")]
fn test_get_query(id: String) -> String {
    id.to_string()
}

#[http_get("get/test/{id}")]
fn test_get_dynamic(id: String) -> String {
    id.to_string()
}

#[http_raw_get("get/test")]
fn test_raw_get() -> String {
    TEST_GET_MESSAGE.to_string()
}

#[http_raw_get("get/test/query")]
fn test_raw_get_query(id: String) -> String {
    id.to_string()
}

// ==============
// http_get tests
// ==============
#[test]
fn http_get_request_pattern_should_return_expected_endpoint() {
    assert_eq!(TEST_GET_ENDPOINT, test_get().request_pattern);
}

#[test]
fn http_get_handler_should_return_expected_response() {
    let expected = Response::ok(TEST_GET_MESSAGE, false);
    let request = Request::new(TEST_GET_ENDPOINT, http::methods::GET, None, HashMap::new());

    let result = (test_get().handler)(request);

    assert_eq!(expected, result);
}

#[test]
fn http_get_handler_should_return_expected_request_query() {
    let get_path = &format!("{TEST_GET_QUERY_ENDPOINT}?id={TEST_GET_ID}");
    let expected = Response::ok(TEST_GET_ID, false);
    let request = Request::new(get_path, http::methods::GET, None, HashMap::new());

    let result = (test_get_query().handler)(request);

    assert_eq!(expected, result);
}

#[test]
fn http_get_handler_should_return_expected_dynamic_path_value() {
    let get_path = &format!("{TEST_GET_ENDPOINT}/{TEST_GET_ID}");
    let expected = Response::ok(TEST_GET_ID, false);
    let request = Request::new(
        get_path,
        http::methods::GET,
        None,
        HashMap::from([("id".to_string(), TEST_GET_ID.to_string())]),
    );

    let result = (test_get_dynamic().handler)(request);

    assert_eq!(expected, result);
}

// ==================
// http_raw_get tests
// ==================
#[test]
fn http_raw_get_request_pattern_should_return_expected_endpoint() {
    assert_eq!(TEST_GET_ENDPOINT, test_raw_get().request_pattern);
}

#[test]
fn http_raw_get_handler_should_return_expected_raw_response() {
    let expected = Response::ok(TEST_GET_MESSAGE, true);
    let request = Request::new(TEST_GET_ENDPOINT, http::methods::GET, None, HashMap::new());

    let result = (test_raw_get().handler)(request);

    assert_eq!(expected, result);
}

#[test]
fn http_raw_get_handler_should_return_expected_request_query() {
    let get_path = &format!("{TEST_GET_QUERY_ENDPOINT}?id={TEST_GET_ID}");
    let expected = Response::ok(TEST_GET_ID, true);
    let request = Request::new(get_path, http::methods::GET, None, HashMap::new());

    let result = (test_raw_get_query().handler)(request);

    assert_eq!(expected, result);
}
