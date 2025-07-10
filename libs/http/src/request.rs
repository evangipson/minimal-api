use crate::{
    constants::HTTP_VERSION,
    methods::{DELETE, GET, POST, PUT},
};

/// [`Request`] represents a web request.
#[derive(PartialEq)]
pub struct Request {
    /// [`Request::path`] is the **entire** [`Request`] path.
    /// # Example values
    /// - `/`
    /// - `/some/path`
    /// - `/some/path?querykey=queryvalue`
    /// - `/some/path#SomeAnchor`
    pub path: String,
    /// [`Request::method`] is a [`String`] representation of the HTTP method for a [`Request`].
    /// # Example values
    /// - `"GET"`
    /// - `"POST"`
    /// - `"PUT"`
    /// - `"DELETE"`
    pub method: String,
    /// [`Request::body_content`] is an optional [`String`] representation of any body content sent as part of a [`Request`].
    pub body_content: Option<String>,
}

impl Request {
    /// [`Request::new`] creates a new [`Request`] and set the [`Request::body_content`] to [`None`].
    /// # Example
    /// [`Request::new`] can be used to create a new `GET` [`Request`] for any path:
    /// ```rust
    /// use http::request::Request;
    ///
    /// fn create_get_request(path: &str) -> Request {
    ///     http::request::Request::new(path, "GET")
    /// }
    /// ```
    pub fn new(path: &str, method: &str) -> Self {
        Request {
            path: path.to_string(),
            method: method.to_string(),
            body_content: None,
        }
    }

    /// [`Request::get`] creates an HTTP `GET` request header using the provided `path`.
    /// # Example
    /// [`Request::get`] can be used to create an HTTP `GET` request header for any path:
    /// ```rust
    /// use http::request::Request;
    ///
    /// fn create_get_header(path: &str) -> String {
    ///     http::request::Request::get(path)
    /// }
    /// ```
    pub fn get(path: &str) -> String {
        format!("{GET} {path} {HTTP_VERSION}")
    }

    /// [`Request::post`] creates an HTTP `POST` request header using the provided `path`.
    /// # Example
    /// [`Request::post`] can be used to create an HTTP `POST` request header for any path:
    /// ```rust
    /// use http::request::Request;
    ///
    /// fn create_post_header(path: &str) -> String {
    ///     http::request::Request::post(path)
    /// }
    /// ```
    pub fn post(path: &str) -> String {
        format!("{POST} {path} {HTTP_VERSION}")
    }

    /// [`Request::put`] creates an HTTP `PUT` request header using the provided `path`.
    /// # Example
    /// [`Request::put`] can be used to create an HTTP `PUT` request header for any path:
    /// ```rust
    /// use http::request::Request;
    ///
    /// fn create_put_header(path: &str) -> String {
    ///     http::request::Request::put(path)
    /// }
    /// ```
    pub fn put(path: &str) -> String {
        format!("{PUT} {path} {HTTP_VERSION}")
    }

    /// [`Request::delete`] creates an HTTP `DELETE` request header using the provided `path`.
    /// # Example
    /// [`Request::delete`] can be used to create an HTTP `DELETE` request header for any path:
    /// ```rust
    /// use http::request::Request;
    ///
    /// fn create_delete_header(path: &str) -> String {
    ///     http::request::Request::delete("/some/path")
    /// }
    /// ```
    pub fn delete(path: &str) -> String {
        format!("{DELETE} {path} {HTTP_VERSION}")
    }

    /// [`Request::query_param`] will get a query parameter value by `name`,
    /// defaults to `None` if the query parameter is not found.
    /// # Example
    /// [`Request::query_param`] can be used to look up a query parameter on
    /// a [`Request`] by a [`str`] key:
    /// ```rust
    /// use http::request::Request;
    ///
    /// fn get_query_param(request: Request, key: &str) -> String {
    ///     if let Some(query_value) = request.query_param(key) {
    ///         query_value.to_string()
    ///     } else {
    ///         "".to_string()
    ///     }
    /// }
    /// ```
    pub fn query_param(&self, name: &str) -> Option<&str> {
        let path_parts: Vec<&str> = self.path.splitn(2, '?').collect();

        // if there is no query string, return None
        if path_parts.len() < 2 {
            return None;
        }

        // get all query strings
        let query_string = path_parts[1];
        for pair in query_string.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                // return the first matched query string value
                if key == name {
                    return Some(value);
                }
            }
        }

        // if there was a query string, but there was no matching value, return None
        None
    }

    /// [`Request::body_as_string`] will return a [`String`] representation
    /// of [`Request::body_content`].
    /// # Example
    /// [`Request::body_as_string`] can be used to get [`Request::body_content`]:
    /// ```rust
    /// use http::request::Request;
    ///
    /// fn get_body_from_request(request: Request) -> String {
    ///     if let Ok(body_content) = request.body_as_string() {
    ///         body_content
    ///     } else {
    ///         String::new()
    ///     }
    /// }
    /// ```
    pub fn body_as_string(&self) -> Result<String, String> {
        self.body_content
            .clone()
            .ok_or_else(|| "Request body not available".to_string())
    }
}
