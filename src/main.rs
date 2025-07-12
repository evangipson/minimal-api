use logger::logger::{LogSeverity, set_logging_severity};
use minimal_api::server::listener::listen;

/// [`main`] is the entry point of [`minimal_api`], which invokes the
/// [`listen`] function to listen for any [`Route`](http::route::Route)
/// that is returned by the
/// [`get_endpoints`](minimal_api::routes::index::get_endpoints) function.
fn main() {
    set_logging_severity(LogSeverity::Warning);
    listen();
}
