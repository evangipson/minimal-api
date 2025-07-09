//! # [`http`](crate)
//! The [`http`](crate) crate contains abstractions such as [`Request`](request::Request),
//! [`Response`](response::Response), and [`Route`](route::Route) to facilitate HTTP
//! communication between a client and a server.

/// [`constants`] is a collection of constant values that represent common HTTP header values.
pub mod constants;

/// [`methods`] is a collection of constant values that represent HTTP methods.
pub mod methods;

/// [`request`] holds all functionality related to HTTP requests.
pub mod request;

/// [`respond`] contains traits to make serving HTTP response content easier.
pub mod respond;

/// [`response`] holds all functionality related to HTTP responses.
pub mod response;

/// [`route`] holds all functionality that will serve a response based on a request.
pub mod route;

/// [`status`] is a collection of HTTP statuses.
pub mod status;
