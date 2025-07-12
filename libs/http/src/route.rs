use crate::{
    methods::{DELETE, GET, POST, PUT},
    request::Request,
    response::Response,
};
use std::collections::HashMap;

/// [`RouteHandler`] is a dynamic handler function for a [`Route`],
/// which takes a [`Request`] and gives back a [`Response`].
pub type RouteHandler = Box<dyn Fn(Request) -> Response + Send + Sync + 'static>;

/// [`Route`] represents routing information and functionality for a server.
pub struct Route {
    /// [`Route::method`] is a [`String`] representation of an HTTP method.
    pub method: String,
    /// [`Route::request_pattern`] is a [`String`] representation of a [`Request`]
    /// path.
    pub request_pattern: String,
    /// [`Route::fallback_responses`] is a collection of [`Response`] for a [`Route`]
    /// to serve in various error cases.
    pub fallback_responses: Vec<Response>,
    /// [`Route::handler`] is a [`RouteHandler`] that returns the intended [`Response`]
    /// for a [`Request`].
    pub handler: RouteHandler,
}

impl Route {
    /// [`Route::get`] creates a [`Route`] that represents an HTTP `GET` [`Request`],
    /// and it's coupled [`Response`].
    /// # Example
    /// [`Route::get`] can be used to create a [`Response`] for an HTTP `GET` [`Request`]:
    /// ```rust
    /// use http::{
    ///     response::Response,
    ///     request::Request,
    ///     route::Route,
    /// };
    ///
    /// fn get_route_handler(_request: Request) -> Response {
    ///     Response::ok("get ok!", false)
    /// }
    ///
    /// fn create_get_route(path: &str) -> Route {
    ///     Route::get(
    ///         path,
    ///         (Box::new(get_route_handler) as http::route::RouteHandler),
    ///     )
    /// }
    /// ```
    pub fn get(path: &str, handler: RouteHandler) -> Self {
        Route::new(GET, path, handler)
    }

    /// [`Route::post`] creates a [`Route`] that represents an HTTP `POST` [`Request`],
    /// and it's coupled [`Response`].
    /// # Example
    /// [`Route::post`] can be used to create a [`Response`] for an HTTP `POST` [`Request`]:
    /// ```rust
    /// use http::{
    ///     response::Response,
    ///     request::Request,
    ///     route::Route,
    /// };
    ///
    /// fn get_route_handler(_request: Request) -> Response {
    ///     Response::ok("post ok!", false)
    /// }
    ///
    /// fn create_post_route(path: &str) -> Route {
    ///     Route::post(
    ///         path,
    ///         (Box::new(get_route_handler) as http::route::RouteHandler),
    ///     )
    /// }
    /// ```
    pub fn post(path: &str, handler: RouteHandler) -> Self {
        Route::new(POST, path, handler)
    }

    /// [`Route::put`] creates a [`Route`] that represents an HTTP `PUT` [`Request`],
    /// and it's coupled [`Response`].
    /// # Example
    /// [`Route::put`] can be used to create a [`Response`] for an HTTP `PUT` [`Request`]:
    /// ```rust
    /// use http::{
    ///     response::Response,
    ///     request::Request,
    ///     route::Route,
    /// };
    ///
    /// fn get_route_handler(_request: Request) -> Response {
    ///     Response::ok("updated!", false)
    /// }
    ///
    /// fn create_put_route(path: &str) -> Route {
    ///     Route::put(
    ///         path,
    ///         (Box::new(get_route_handler) as http::route::RouteHandler),
    ///     )
    /// }
    /// ```
    pub fn put(path: &str, handler: RouteHandler) -> Self {
        Route::new(PUT, path, handler)
    }

    /// [`Route::delete`] creates a [`Route`] that represents an HTTP `DELETE` [`Request`],
    /// and it's coupled [`Response`].
    /// # Example
    /// [`Route::delete`] can be used to create a [`Response`] for an HTTP `DELETE` [`Request`]:
    /// ```rust
    /// use http::{
    ///     response::Response,
    ///     request::Request,
    ///     route::Route,
    /// };
    ///
    /// fn get_route_handler(_request: Request) -> Response {
    ///     Response::ok("deleted!", false)
    /// }
    ///
    /// fn create_delete_route(path: &str) -> Route {
    ///     Route::delete(
    ///         path,
    ///         (Box::new(get_route_handler) as http::route::RouteHandler),
    ///     )
    /// }
    /// ```
    pub fn delete(path: &str, handler: RouteHandler) -> Self {
        Route::new(DELETE, path, handler)
    }

    /// [`Route::matches_path`] checks if the `request_path` matches this route's pattern and
    /// extracts path parameters, and if so, returns [`Some`] [`HashMap`]. Defaults to [`None`].
    /// # Example
    /// [`Route::matches_path`] can be used to determine if a request path contains any matches
    /// for a [`Route::request_pattern`]:
    /// ```rust
    /// use http::route::Route;
    /// use std::collections::HashMap;
    ///
    /// fn check_endpoint_for_route_match(route: Route, endpoint: &str) -> bool {
    ///     route.matches_path(endpoint).is_some()
    /// }
    /// ```
    pub fn matches_path(&self, request_path: &str) -> Option<HashMap<String, String>> {
        let pattern_segments: Vec<&str> = self.request_pattern.split('/').collect();
        let request_segments: Vec<&str> = request_path.split('/').collect();

        // must have the same number of path segments
        if pattern_segments.len() != request_segments.len() {
            return None;
        }

        let mut path_params = HashMap::new();

        // iterate through segments, comparing static parts and extracting dynamic ones
        for i in 0..pattern_segments.len() {
            let pattern_segment = pattern_segments[i];
            let request_segment = request_segments[i];

            if pattern_segment.starts_with('{') && pattern_segment.ends_with('}') {
                // this is a path parameter (e.g., "{id}")
                let param_name = &pattern_segment[1..pattern_segment.len() - 1]; // Extract "id"
                path_params.insert(param_name.to_string(), request_segment.to_string());
            } else if pattern_segment != request_segment {
                // static segment mismatch (e.g., "/get/" vs "/post/")
                return None;
            }
        }

        Some(path_params)
    }

    /// [`Route::get_response`] will get a [`Response`] based on the provided [`Request`].
    /// # Example
    /// [`Route::get_response`] can be used to get a [`Response`] for a [`Request`]:
    /// ```rust
    /// use http::{
    ///     response::Response,
    ///     request::Request,
    ///     route::Route
    /// };
    ///
    /// fn get_route_response(route: Route, request:Request) -> Response {
    ///     route.get_response(request)
    /// }
    /// ```
    pub fn get_response(&self, request: Request) -> Response {
        (self.handler)(request)
    }

    /// [`Route::new`] creates a new [`Route`] for any `http_method`, which uses the
    /// `path` and `handler` to determine what happens when the [`Route`] is matched
    /// by the server.
    fn new(http_method: &str, path: &str, handler: RouteHandler) -> Self {
        Route {
            request_pattern: path.to_string(),
            method: http_method.to_string(),
            handler,
            fallback_responses: vec![
                Response::not_found(),
                Response::bad_request(),
                Response::unprocessable_entity(),
                Response::server_error(),
            ],
        }
    }
}
