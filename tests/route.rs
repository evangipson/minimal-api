use http::{request::Request, response::Response, route::Route};
use std::collections::HashMap;

// ====================
// common route handler
// ====================
fn route_handler(request: Request) -> Response {
    Response::ok(&request.method, false)
}

#[test]
fn get_route_should_assign_contents_and_provide_get_response() {
    let expected = "get/path";
    let method = http::methods::GET;
    let request = Request::new(expected, method, None, HashMap::new());
    let route = Route::get(expected, Box::new(route_handler));

    let result = route.get_response(request);

    assert_eq!(expected, route.request_pattern);
    assert_eq!(method, result.content);
}

#[test]
fn post_route_should_assign_contents_and_provide_post_response() {
    let expected = "post/path";
    let method = http::methods::POST;
    let request = Request::new(expected, method, None, HashMap::new());
    let route = Route::post(expected, Box::new(route_handler));

    let result = route.get_response(request);

    assert_eq!(expected, route.request_pattern);
    assert_eq!(method, result.content);
}

#[test]
fn put_route_should_assign_contents_and_provide_put_response() {
    let expected = "put/path";
    let method = http::methods::PUT;
    let request = Request::new(expected, method, None, HashMap::new());
    let route = Route::put(expected, Box::new(route_handler));

    let result = route.get_response(request);

    assert_eq!(expected, route.request_pattern);
    assert_eq!(method, result.content);
}

#[test]
fn delete_route_should_assign_contents_and_provide_delete_response() {
    let expected = "delete/path";
    let method = http::methods::DELETE;
    let request = Request::new(expected, method, None, HashMap::new());
    let route = Route::delete(expected, Box::new(route_handler));

    let result = route.get_response(request);

    assert_eq!(expected, route.request_pattern);
    assert_eq!(method, result.content);
}
