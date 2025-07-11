use http::request::Request;

#[test]
fn new_request_should_assign_inputs_as_members() {
    let path = "path";
    let method = "method";

    let request = Request::new(path, method);

    assert_eq!(path, request.path);
    assert_eq!(method, request.method);
}

#[test]
fn new_request_should_have_no_body_content() {
    let request = Request::new("", "");

    assert_eq!(None, request.body_content);
}

#[test]
fn get_request_should_have_valid_http_get_header() {
    let path = "path";
    let get_request = Request::get(path);

    assert!(get_request.contains(http::methods::GET));
    assert!(get_request.contains(path));
    assert!(get_request.contains("HTTP/"));
}

#[test]
fn post_request_should_have_valid_http_post_header() {
    let path = "path";
    let post_request = Request::post(path);

    assert!(post_request.contains(http::methods::POST));
    assert!(post_request.contains(path));
    assert!(post_request.contains("HTTP/"));
}

#[test]
fn put_request_should_have_valid_http_put_header() {
    let path = "path";
    let put_request = Request::put(path);

    assert!(put_request.contains(http::methods::PUT));
    assert!(put_request.contains(path));
    assert!(put_request.contains("HTTP/"));
}

#[test]
fn delete_request_should_have_valid_http_delete_header() {
    let path = "path";
    let delete_request = Request::delete(path);

    assert!(delete_request.contains(http::methods::DELETE));
    assert!(delete_request.contains(path));
    assert!(delete_request.contains("HTTP/"));
}

#[test]
fn query_param_should_return_none_without_any_query() {
    let request = Request::new("", "");

    let result = request.query_param("fake-key");

    assert_eq!(None, result);
}

#[test]
fn query_param_should_return_none_without_query_match() {
    let request = Request::new("?key=value", "");

    let result = request.query_param("other-key");

    assert_eq!(None, result);
}

#[test]
fn query_param_should_return_value_with_valid_query() {
    let key = "key";
    let expected = "value";
    let request = Request::new(&format!("?{key}={expected}"), "");

    let result = request.query_param(key);

    assert!(result.is_some());
    assert_eq!(expected, result.unwrap());
}

#[test]
fn body_as_string_should_error_without_request_body() {
    let request = Request::new("", "");

    let result = request.body_as_string();

    assert!(result.is_err());
}

#[test]
fn body_as_string_should_return_body_content_with_request_body() {
    let expected = "body content";
    let mut request = Request::new("", "");
    request.body_content = Some(expected.to_string());

    let result = request.body_as_string();

    assert!(result.is_ok());
    assert_eq!(expected, result.unwrap());
}
