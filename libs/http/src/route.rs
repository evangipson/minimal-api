use crate::{
    methods::{DELETE, GET, POST, PUT},
    request::Request,
    response::Response,
};

pub struct Route {
    pub request: String,
    pub actions: Vec<Response>,
}

impl Route {
    pub fn get(path: &str, ok_response: String) -> Self {
        Route::new(GET, path, &ok_response)
    }

    pub fn post(path: &str, ok_response: &str) -> Self {
        Route::new(POST, path, ok_response)
    }

    pub fn put(path: &str, ok_response: &str) -> Self {
        Route::new(PUT, path, ok_response)
    }

    pub fn delete(path: &str, ok_response: &str) -> Self {
        Route::new(DELETE, path, ok_response)
    }

    pub fn get_primary_route(&self) -> Response {
        self.actions.first().unwrap().clone()
    }

    fn new(http_method: &str, path: &str, ok_response: &str) -> Self {
        Route {
            request: match http_method {
                GET => Request::get(path),
                POST => Request::post(path),
                PUT => Request::put(path),
                DELETE => Request::delete(path),
                _ => Request::get(path),
            },
            actions: vec![
                Response::ok(ok_response),
                Response::not_found(),
                Response::bad_request(),
                Response::unprocessable_entity(),
                Response::server_error(),
            ],
        }
    }
}
