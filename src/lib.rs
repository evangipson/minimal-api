//! # [`minimal_api`](crate)
//! The [`minimal_api`](crate) crate provides an out-of-the-box solution for
//! hosting a thread-safe web API capable of receiving `GET`, `POST`, `PUT`,
//! and `DELETE` HTTP requests, and serving back content based on a route.
//!
//! It contains no dependencies besides a couple libraries that are included
//! with it:
//! - [`http`] for basic HTTP communication abstractions
//! - [`http_attributes`] for helpful attribute macros, such as
//!   [`http_get`](macro@http_attributes::http_get) and
//!   [`http_post`](macro@http_attributes::http_post), to facilitate creating
//!   routes.
//! - [`logger`] for helpful logging macros, such as
//!   [`log_info`](macro@logger::log_info) and [`log_error`](macro@logger::log_error).
//!
//! # Creating routes
//! The following example sets up an endpoint at the index route (`/`) that
//! returns "Hello!":
//! ```rust
//! use http_attributes::http_get;
//!
//! #[http_get("/")]
//! pub fn say_hello() -> String {
//!     format!("Hello!")
//! }
//! ```
//!
//! The following example sets up an endpoint at the `/who` endpoint that returns
//! a message with the value of the `name` query parameter:
//! ```rust
//! use http_attributes::http_get;
//!
//! #[http_get("/who")]
//! pub fn say_hello(name: String) -> String {
//!     format!("Hello, {name}!")
//! }
//! ```
//!
//! The following example sets up a `GET` endpoint for the `/user` path that
//! returns a message with the value of the `id` path segment:
//! ```rust
//! use http_attributes::http_get;
//!
//! #[http_get("/user/{id}")]
//! pub fn get_user(id: String) -> String {
//!     format!("Found user by id '{id}'!")
//! }
//! ```
//!
//! The following example sets up a `POST` endpoint for the `/submit` path that
//! returns the `POST` data:
//! ```rust
//! use http_attributes::http_post;
//!
//! #[http_post("/submit")]
//! pub fn get_post_data(content: String) -> String {
//!     format!("Received '{content}' from POST")
//! }
//! ```
//!
//! The following example sets up a `PUT` endpoint for the `/update` path that
//! returns the `PUT` data:
//! ```rust
//! use http_attributes::http_put;
//!
//! #[http_put("/update")]
//! pub fn get_put_data(content: String) -> String {
//!     format!("Received '{content}' from PUT")
//! }
//! ```
//!
//! The following example sets up a `DELETE` endpoint for the `/remove` path
//! that returns a query
//! parameter value sent to the `DELETE` route:
//! ```rust
//! use http_attributes::http_delete;
//!
//! #[http_delete("/remove")]
//! pub fn get_delete_id(id: String) -> String {
//!     format!("Received '{id}' from DELETE")
//! }
//! ```

/// [`server`] contains all functionality related to handling client requests
/// and serving responses safely.
pub mod server {
    /// [`job`] contains type definitions for the server.
    pub mod job;
    /// [`listener`] contains all functionality for how the server listens
    /// for requests.
    pub mod listener;
    /// [`thread_pool`] contains a basic thread pool implementation to allow
    /// the server to be multi-threaded.
    pub mod thread_pool;
    /// [`worker`] contains all functionality for how the server dispatches
    /// it's responses.
    pub mod worker;
}

/// [`environment`] contains all environment variables, to make
/// [`minimal_api`](crate) easier to host locally or through Docker.
pub mod environment {
    /// [`app`] contains all environment variables that are application-centric,
    /// like version number and application name.
    pub mod app;
    /// [`server`] contains all environment variables that are specifically for
    /// the web server.
    pub mod server;
}

/// [`routes`] contains all routing information for the server.
pub mod routes {
    /// [`mock`] represents a more realistic application with more complex routes.
    pub mod mock {
        /// [`base_response`] contains base-level response information for a
        /// [`mock`](crate::routes::mock) route.
        pub mod base_response;
        /// [`session`] contains all [`mock`](crate::routes::mock) routes that
        /// are prefixed with `/session`.
        pub mod session;
    }
    /// [`index`] holds a very important function,
    /// [`get_endpoints`](crate::routes::index::get_endpoints), which contains
    /// all endpoints that the server will know about.
    pub mod index;
}
