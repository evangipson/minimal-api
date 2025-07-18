use crate::{
    environment::app::{CRATE_NAME, CRATE_VERSION},
    routes,
};
use http::{respond::Respond, route::Route};
use http_attributes::{http_delete, http_get, http_post, http_put};

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
pub fn show_number_squared(number: String) -> String {
    if number.parse::<i32>().is_ok() {
        let parsed_number: i32 = number.parse().unwrap();
        let squared_number = parsed_number * parsed_number;
        format!("{parsed_number} squared is {squared_number}.").get_json()
    } else {
        format!("{number} is not a number.").get_json()
    }
}

#[http_get("/user/{id}")]
pub fn show_user_by_id(id: String) -> String {
    format!("Found user by id '{id}'").get_json()
}

#[http_post("/sendname")]
pub fn get_name(name: String) -> String {
    format!("Received name '{name}' from POST route!").get_json()
}

#[http_put("/update")]
pub fn get_update_id(id: String) -> String {
    format!("Received id '{id}' from PUT route!").get_json()
}

#[http_delete("/remove")]
pub fn get_delete_id(id: String) -> String {
    format!("Received id '{id}' from DELETE route!").get_json()
}

/// [`get_endpoints`] will return a collection of [`Route`] for the
/// server to listen for and respond to.
pub fn get_endpoints() -> Vec<Route> {
    vec![
        show_welcome_message(),
        show_crate_name(),
        show_crate_version(),
        show_number_squared(),
        show_user_by_id(),
        get_name(),
        get_update_id(),
        get_delete_id(),
        routes::mock::session::create_new_session_id(),
    ]
}
