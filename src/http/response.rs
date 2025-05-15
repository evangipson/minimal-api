use crate::time::date::Date;
use std::collections::HashMap;

const HTTP_VERSION: &str = "HTTP/1.1";
const CONTENT_LENGTH: &str = "Content-Length";
const CONTENT_TYPE: &str = "Content-Type";
const CONTENT_JSON: &str = "application/json";

pub enum Status {
    OK,
    NOTFOUND,
}

impl Status {
    pub const OK: Status = Status::OK;
    pub const NOTFOUND: Status = Status::NOTFOUND;
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Status::OK => "200 OK",
            Status::NOTFOUND => "404 NOT FOUND",
        };
        write!(f, "{status}")
    }
}

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

pub trait Respond {
    fn get_json(&self) -> String;
}

impl Respond for String {
    fn get_json(&self) -> String {
        "\"".to_string() + self + "\""
    }
}

impl<T: std::string::ToString, U: std::string::ToString> Respond for HashMap<T, U> {
    fn get_json(&self) -> String {
        "{".to_owned()
            + self
                .iter()
                .map(|(t, u)| "\"".to_string() + &t.to_string() + "\":\"" + &u.to_string() + "\", ")
                .collect::<String>()
                .trim_end_matches(",")
            + "}"
    }
}
