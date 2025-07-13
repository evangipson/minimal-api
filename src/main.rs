use logger::log_severity::LogSeverity;
use minimal_api::server::listener;

/// [`main`] is the entry point of [`minimal_api`], which invokes the
/// [`listen`] function to listen for any [`Route`](http::route::Route)
/// that is returned by the
/// [`get_endpoints`](minimal_api::routes::index::get_endpoints) function.
fn main() {
    logger::set_logging_severity(LogSeverity::Debug);
    listener::listen();
}
