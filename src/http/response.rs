use crate::{
    http::{
        constants::{CONTENT_JSON, CONTENT_LENGTH, CONTENT_TYPE, HTTP_VERSION},
        respond::Respond,
        status::Status,
    },
    time::date::Date,
};
use std::collections::HashMap;

pub struct Response {
    pub content: String,
    pub status: Status,
    pub time: Date,
}

impl Response {
    pub fn ok(contents: &str) -> String {
        let response = Self::string_response(contents.to_string(), Status::OK);
        format!(
            "{HTTP_VERSION} {}\r\n{CONTENT_LENGTH}: {}\r\n{CONTENT_TYPE}: {CONTENT_JSON}\r\n\r\n{}",
            response.status,
            response.len(),
            response.respond()
        )
    }

    pub fn not_found() -> String {
        let response = Self::string_response(
            "That doesn't exist on the menu.".to_string(),
            Status::NOTFOUND,
        );
        format!(
            "{HTTP_VERSION} {}\r\n{CONTENT_LENGTH}: {}\r\n{CONTENT_TYPE}: {CONTENT_JSON}\r\n\r\n{}",
            response.status,
            response.len(),
            response.respond()
        )
    }

    pub fn string_response(content: String, status: Status) -> Self {
        Response {
            content: content.get_json(),
            status,
            time: Date::new(),
        }
    }

    pub fn object_response<T: std::string::ToString, U: std::string::ToString>(
        content: HashMap<T, U>,
        status: Status,
    ) -> Self {
        Response {
            content: content.get_json(),
            status,
            time: Date::new(),
        }
    }

    fn respond(&self) -> String {
        "{".to_owned()
            + &format!(
                "\"content\":{},\"status\":\"{}\",\"time\":\"{}\"",
                self.content, self.status, self.time.formatted
            )
            + "}"
    }

    pub fn len(&self) -> usize {
        self.respond().len()
    }

    pub fn is_empty(&self) -> bool {
        self.respond().len() == 0
    }
}
