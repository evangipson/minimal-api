use crate::{
    constants::{CONTENT_JSON, CONTENT_LENGTH, CONTENT_TYPE, HTTP_VERSION},
    status::Status,
};
use time::date::Date;

#[derive(Clone)]
pub struct Response {
    pub header: String,
    pub content: String,
    pub status: Status,
    pub time: Date,
}

impl Response {
    pub fn ok(contents: &str) -> Self {
        Response::new(Status::Ok, contents)
    }

    pub fn not_found() -> Self {
        Response::new(Status::NotFound, "That doesn't exist on the menu")
    }

    pub fn bad_request() -> Self {
        Response::new(Status::BadRequest, "Are you sure about that?")
    }

    pub fn unprocessable_entity() -> Self {
        Response::new(Status::UnprocessableEntity, "I don't know what to do.")
    }

    pub fn server_error() -> Self {
        Response::new(Status::ServerError, "The server did not like that one")
    }

    fn new(status: Status, contents: &str) -> Self {
        Response {
            content: contents.to_string(),
            status,
            time: Date::new(),
            header: String::new(),
        }
        .add_http_headers()
    }

    // fn object_response<T: std::string::ToString, U: std::string::ToString>(
    //     content: HashMap<T, U>,
    //     status: Status,
    // ) -> Self {
    //     Response {
    //         content: content.get_json(),
    //         status,
    //         time: Date::new(),
    //         header: String::new(),
    //     }
    //     .add_http_headers()
    // }

    fn add_http_headers(mut self) -> Self {
        self.header = format!(
            "{HTTP_VERSION} {}\r\n{CONTENT_LENGTH}: {}\r\n{CONTENT_TYPE}: {CONTENT_JSON}\r\n\r\n{}",
            self.status,
            self.len(),
            self.respond()
        );
        self
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

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)
    }
}
