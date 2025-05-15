use crate::http::{constants::HTTP_VERSION, methods::GET};

#[derive(PartialEq)]
pub struct Request {
    pub path: String,
    pub method: String,
}

impl Request {
    pub fn new(path: &str, method: &str) -> Self {
        Request {
            path: path.to_string(),
            method: method.to_string(),
        }
    }

    pub fn get(path: &str) -> String {
        format!("{GET} {path} {HTTP_VERSION}")
    }
}
