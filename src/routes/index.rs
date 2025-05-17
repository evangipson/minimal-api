use crate::environment::app::{CRATE_NAME, CRATE_VERSION};
use attributes::http_get;
use http::route::Route;

#[http_get("/")]
pub fn show_welcome_message() -> String {
    format!("Hello from {} v{}!", CRATE_NAME, CRATE_VERSION)
}

#[http_get("/name")]
pub fn show_crate_name() -> String {
    format!("{}", CRATE_NAME)
}

#[http_get("/version")]
pub fn show_crate_version() -> String {
    format!("{}", CRATE_VERSION)
}

pub fn get_endpoints() -> Vec<Route> {
    vec![
        show_welcome_message(),
        show_crate_name(),
        show_crate_version(),
    ]
}
