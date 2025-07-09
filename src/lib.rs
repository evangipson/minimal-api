//! # [`minimal_api`](crate)
//! The [`minimal_api`](crate) crate provides an out-of-the-box solution for
//! hosting a thread-safe web API capable of receiving `GET`, `POST`, `PUT`,
//! and `DELETE` HTTP requests, and serving back content based on a route.
//!
//! It contains no dependencies besides a couple libraries that are included
//! with it:
//! - [`http`] for basic HTTP communication abstractions
//! - [`attributes`] for helpful attribute macros, such as
//!   [`http_get`](macro@attributes::http_get) and
//!   [`http_post`](macro@attributes::http_post), to facilitate creating routes.
//!
//!
//! # Setting up Endpoints
//! The following example sets up an endpoint at the index route (`/`) that
//! returns "Hello!":
//! ```rust
//! use attributes::http_get;
//! use http::route::Route;
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
//! use attributes::http_get;
//! use http::route::Route;
//!
//! #[http_get("/who")]
//! pub fn say_hello(name: &str) -> String {
//!     format!("Hello, {name}!")
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
