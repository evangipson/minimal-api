use http::{request::Request, response::Response};
use http_attributes::{http_delete, http_raw_delete};
use std::collections::HashMap;

// ==================
// common test values
// ==================
const TEST_DELETE_ID: &str = "DELETE-ME";
const TEST_DELETE_ENDPOINT: &str = "delete/test";

// =================
// endpoints to test
// =================
#[http_delete("delete/test")]
fn test_delete(id: String) -> String {
    id.to_string()
}

#[http_delete("delete/test/{id}")]
fn test_delete_dynamic(id: String) -> String {
    id.to_string()
}

#[http_raw_delete("delete/test")]
fn test_raw_delete(id: String) -> String {
    id.to_string()
}

#[http_raw_delete("delete/test/{id}")]
fn test_raw_delete_dynamic(id: String) -> String {
    id.to_string()
}

// =================
// http_delete tests
// =================
#[test]
fn http_delete_request_pattern_should_return_expected_endpoint() {
    assert_eq!(TEST_DELETE_ENDPOINT, test_delete().request_pattern);
}

#[test]
fn http_delete_handler_should_return_expected_response() {
    let delete_path = &format!("{TEST_DELETE_ENDPOINT}?id={TEST_DELETE_ID}");
    let expected = Response::ok(TEST_DELETE_ID, false);
    let request = Request::new(delete_path, http::methods::DELETE, None, HashMap::new());

    let result = test_delete().get_response(request);

    assert_eq!(expected, result);
}

#[test]
fn http_delete_handler_should_return_expected_dynamic_path_value() {
    let expected = Response::ok(TEST_DELETE_ID, false);
    let request = Request::new(
        TEST_DELETE_ENDPOINT,
        http::methods::DELETE,
        None,
        HashMap::from([("id".to_string(), TEST_DELETE_ID.to_string())]),
    );

    let result = test_delete_dynamic().get_response(request);

    assert_eq!(expected, result);
}

// =====================
// http_raw_delete tests
// =====================
#[test]
fn http_raw_delete_request_pattern_should_return_expected_endpoint() {
    assert_eq!(TEST_DELETE_ENDPOINT, test_raw_delete().request_pattern);
}

#[test]
fn http_raw_delete_handler_should_return_expected_raw_response() {
    let delete_path = &format!("{TEST_DELETE_ENDPOINT}?id={TEST_DELETE_ID}");
    let expected = Response::ok(TEST_DELETE_ID, true);
    let request = Request::new(delete_path, http::methods::DELETE, None, HashMap::new());

    let result = test_raw_delete().get_response(request);

    assert_eq!(expected, result);
}

#[test]
fn http_raw_delete_handler_should_return_expected_dynamic_path_value() {
    let expected = Response::ok(TEST_DELETE_ID, true);
    let request = Request::new(
        TEST_DELETE_ENDPOINT,
        http::methods::DELETE,
        None,
        HashMap::from([("id".to_string(), TEST_DELETE_ID.to_string())]),
    );

    let result = test_raw_delete_dynamic().get_response(request);

    assert_eq!(expected, result);
}
