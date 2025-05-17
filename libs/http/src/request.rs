use crate::{
    constants::HTTP_VERSION,
    methods::{DELETE, GET, POST, PUT},
};

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

    pub fn post(path: &str) -> String {
        format!("{POST} {path} {HTTP_VERSION}")
    }

    pub fn put(path: &str) -> String {
        format!("{PUT} {path} {HTTP_VERSION}")
    }

    pub fn delete(path: &str) -> String {
        format!("{DELETE} {path} {HTTP_VERSION}")
    }
}
