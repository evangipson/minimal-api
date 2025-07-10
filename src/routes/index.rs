use crate::{
    environment::app::{CRATE_NAME, CRATE_VERSION},
    routes,
};
use attributes::{http_get, http_post};
use http::{respond::Respond, route::Route};

#[http_get("/")]
pub fn show_welcome_message() -> String {
    format!("Hello from {CRATE_NAME} v{CRATE_VERSION}!").get_json()
}

#[http_get("/name")]
pub fn show_crate_name() -> String {
    CRATE_NAME.get_json()
}

#[http_get("/version")]
pub fn show_crate_version() -> String {
    CRATE_VERSION.get_json()
}

#[http_get("/squared")]
pub fn show_number_squared(number: i32) -> String {
    let squared_number = number * number;
    squared_number.to_string().get_json()
}

#[http_post("/sendname")]
pub fn get_name(name: String) -> String {
    name.to_string().get_json()
}

pub fn get_endpoints() -> Vec<Route> {
    vec![
        show_welcome_message(),
        show_crate_name(),
        show_crate_version(),
        show_number_squared(),
        get_name(),
        routes::mock::session::create_new_session_id(),
    ]
}
