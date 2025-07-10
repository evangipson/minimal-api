use crate::routes::mock::base_response::BaseMockResponse;
use http::respond::Respond;
use http_attributes::http_raw_get;
use std::collections::HashMap;

#[http_raw_get("/Services/Session/GenerateSessionId")]
pub fn create_new_session_id() -> String {
    std::iter::once((
        "SessionId",
        Box::new("1234567890ABCDEF") as Box<dyn Respond>,
    ))
    .chain(BaseMockResponse::get_default_response())
    .collect::<HashMap<&str, Box<dyn Respond>>>()
    .get_json()
}
