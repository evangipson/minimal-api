use http::response::Response;
use http::status::Status;

#[test]
fn ok_response_should_assign_contents_and_have_ok_status() {
    let expected = "ok";

    let result = Response::ok(expected, false);

    assert_eq!(expected, result.content);
    assert_eq!(Status::Ok, result.status);
}

#[test]
fn not_found_response_should_have_not_found_status() {
    let result = Response::not_found();

    assert_eq!(Status::NotFound, result.status);
}

#[test]
fn bad_request_response_should_have_bad_request_status() {
    let result = Response::bad_request();

    assert_eq!(Status::BadRequest, result.status);
}

#[test]
fn unprocessable_entity_response_should_have_unprocessable_entity_status() {
    let result = Response::unprocessable_entity();

    assert_eq!(Status::UnprocessableEntity, result.status);
}

#[test]
fn server_error_response_should_have_server_error_status() {
    let result = Response::server_error();

    assert_eq!(Status::ServerError, result.status);
}
