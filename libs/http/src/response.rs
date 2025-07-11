use crate::{
    constants::{CONTENT_JSON, CONTENT_LENGTH, CONTENT_TYPE, HTTP_VERSION},
    status::Status,
};
use std::{io::Write, net::TcpStream};
use time::date::Date;

/// [`Response`] represents a response to a web request.
#[derive(Clone, Debug, PartialEq)]
pub struct Response {
    /// [`Response::header`] is the HTTP header of a response.
    /// # Example values
    /// - `GET / HTTP/1.1`
    /// - `POST /data HTTP/1.1`
    pub header: String,
    /// [`Response::content`] is the content returned by a response.
    pub content: String,
    /// [`Response::status`] is the HTTP status of a response.
    /// # Example values
    /// - [`Status::Ok`]
    /// - [`Status::NotFound`]
    pub status: Status,
    /// [`Response::time`] is a timestamp of when a response is served.
    pub time: Date,
}

impl Response {
    /// [`Response::ok`] represents a [`Response`] with [`Status::OK`].
    /// # Example
    /// [`Response::ok`] can be used to create a [`Response`] that returns a
    /// simple [`String`] with [`Status::OK`]:
    /// ```rust
    /// use http::response::Response;
    ///
    /// fn create_simple_ok_response() -> Response {
    ///     Response::ok("ok, got your request!", false)
    /// }
    /// ```
    pub fn ok(contents: &str, raw_response: bool) -> Self {
        Response::new(Status::Ok, contents, raw_response)
    }

    /// [`Response::not_found`] represents a [`Status::NotFound`] [`Response`].
    /// # Example
    /// [`Response::not_found`] can be used to create a [`Response`] that
    /// returns a simple [`String`] with [`Status::NotFound`]:
    /// ```rust
    /// use http::response::Response;
    ///
    /// fn create_simple_not_found_response() -> Response {
    ///     Response::not_found()
    /// }
    /// ```
    pub fn not_found() -> Self {
        Response::new(
            Status::NotFound,
            "\"That doesn't exist on the menu\"",
            false,
        )
    }

    /// [`Response::bad_request`] represents a [`Status::BadRequest`] [`Response`].
    /// # Example
    /// [`Response::bad_request`] can be used to create a [`Response`] that
    /// returns a simple [`String`] with [`Status::BadRequest`]:
    /// ```rust
    /// use http::response::Response;
    ///
    /// fn create_simple_bad_request_response() -> Response {
    ///     Response::bad_request()
    /// }
    /// ```
    pub fn bad_request() -> Self {
        Response::new(Status::BadRequest, "\"Are you sure about that?\"", false)
    }

    /// [`Response::unprocessable_entity`] represents a [`Status::UnprocessableEntity`]
    /// [`Response`].
    /// # Example
    /// [`Response::unprocessable_entity`] can be used to create a [`Response`] that
    /// returns a simple [`String`] with [`Status::UnprocessableEntity`]:
    /// ```rust
    /// use http::response::Response;
    ///
    /// fn create_simple_unprocessable_entity_response() -> Response {
    ///     Response::unprocessable_entity()
    /// }
    /// ```
    pub fn unprocessable_entity() -> Self {
        Response::new(
            Status::UnprocessableEntity,
            "\"I don't know how to handle those inputs, Hal.\"",
            false,
        )
    }

    /// [`Response::server_error`] represents a [`Status::ServerError`] [`Response`].
    /// # Example
    /// [`Response::server_error`] can be used to create a [`Response`] that
    /// returns a simple [`String`] with [`Status::ServerError`]:
    /// ```rust
    /// use http::response::Response;
    ///
    /// fn create_simple_server_error_response() -> Response {
    ///     Response::server_error()
    /// }
    /// ```
    pub fn server_error() -> Self {
        Response::new(
            Status::ServerError,
            "\"The server did not like that one\"",
            false,
        )
    }

    /// [`Response::server_error`] creates a [`Response`] with the provided
    /// [`Status`].
    fn new(status: Status, contents: &str, raw_response: bool) -> Self {
        Response {
            content: contents.to_string(),
            status,
            time: Date::new(),
            header: String::new(),
        }
        .add_http_headers(raw_response)
    }

    /// [`Response::add_http_headers`] adds [`Response::header`] information
    /// to a [`Response`].
    fn add_http_headers(mut self, raw_response: bool) -> Self {
        self.header = format!(
            "{HTTP_VERSION} {}\r\n{CONTENT_LENGTH}: {}\r\n{CONTENT_TYPE}: {CONTENT_JSON}\r\n\r\n{}",
            self.status,
            self.len(raw_response),
            self.render_body(raw_response)
        );
        self
    }

    /// [`Response::render_body`] returns a JSON [`String`] representation of
    /// [`Response::content`].
    fn render_body(&self, raw_response: bool) -> String {
        if raw_response {
            self.content.clone()
        } else {
            format!(
                r#"{{"content":{},"status":"{}","time":"{}"}}"#,
                self.content, self.status, self.time.formatted
            )
        }
    }

    /// [`Response::len`] will return the length of the JSON [`String`]
    /// representation of [`Response::content`].
    /// # Example
    /// [`Response::len`] can be used to determine the size of
    /// [`Response::content`] as it will be returned by the server:
    /// ```rust
    /// use http::response::Response;
    ///
    /// fn get_response_length(response: Response) -> usize {
    ///     response.len(false)
    /// }
    /// ```
    pub fn len(&self, raw_response: bool) -> usize {
        self.render_body(raw_response).len()
    }

    /// [`Response::is_empty`] will return `true` if the [`Response::content`]
    /// is empty, and `false` otherwise.
    /// # Example
    /// [`Response::is_empty`] can be used to determine if a [`Response`] has
    /// any [`Response::content`]:
    /// ```rust
    /// use http::response::Response;
    ///
    /// fn response_content_exists(response: Response) -> bool {
    ///     response.is_empty(false)
    /// }
    /// ```
    pub fn is_empty(&self, raw_response: bool) -> bool {
        self.render_body(raw_response).len() == 0
    }

    /// [`Response::send`] will create a well-formed HTTP result, and write that
    /// result to the provided [`TcpStream`], then return an [`Ok`].
    /// # Example
    /// [`Response::send`] can be used to send an HTTP response back to a
    /// [`TcpStream`]:
    /// ```rust
    /// use http::response::Response;
    /// use std::net::TcpStream;
    ///
    /// fn send_response_to_tcp_stream(
    ///     response: Response,
    ///     stream: &mut TcpStream
    /// ) -> std::io::Result<()> {
    ///     response.send(stream, false)
    /// }
    /// ```
    pub fn send(&self, stream: &mut TcpStream, raw_response: bool) -> std::io::Result<()> {
        let body = self.render_body(raw_response);
        let body_length = body.len();

        // create the status line and headers
        let response_start = format!(
            "{HTTP_VERSION} {}\r\n{CONTENT_LENGTH}: {}\r\n{CONTENT_TYPE}: {CONTENT_JSON}\r\n\r\n",
            self.status, body_length
        );

        // write headers and body separately
        stream.write_all(response_start.as_bytes())?;
        stream.write_all(body.as_bytes())?;
        stream.flush()?;

        Ok(())
    }
}

/// Implement [`std::fmt::Display`] for [`Response`]
impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)
    }
}
