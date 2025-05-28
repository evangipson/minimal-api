use crate::environment::app::{CRATE_NAME, CRATE_VERSION};
use attributes::http_get;
use http::{respond::Respond, route::Route};
use std::collections::HashMap;

#[http_get("/")]
pub fn show_welcome_message() -> String {
    format!("Hello from {} v{}!", CRATE_NAME, CRATE_VERSION).get_json()
}

#[http_get("/name")]
pub fn show_crate_name() -> String {
    CRATE_NAME.get_json()
}

#[http_get("/version")]
pub fn show_crate_version() -> String {
    CRATE_VERSION.get_json()
}

#[http_get("/complex")]
pub fn show_complex_object() -> String {
    HashMap::from([("session_id", "13513ijgf")]).get_json()
}

pub fn get_endpoints() -> Vec<Route> {
    vec![
        show_welcome_message(),
        show_crate_name(),
        show_crate_version(),
        show_complex_object(),
    ]
}
