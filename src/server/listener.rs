use crate::{
    environment::{
        app::{CRATE_NAME, CRATE_VERSION},
        server::ServerConfig,
    },
    http::{request::Request, response::Response},
    server::thread_pool::ThreadPool,
};
use std::{
    collections::HashMap,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    sync::OnceLock,
};

// use OnceLock to only initialize the static collection of endpoints once in a thread-safe manner
static ENDPOINTS: OnceLock<HashMap<String, String>> = OnceLock::new();
fn get_endpoints() -> &'static HashMap<String, String> {
    ENDPOINTS.get_or_init(|| {
        [
            (
                Request::get("/"),
                Response::ok(&format!("Hello from {} v{}!", CRATE_NAME, CRATE_VERSION)),
            ),
            (Request::get("/name"), Response::ok(CRATE_NAME)),
            (Request::get("/version"), Response::ok(CRATE_VERSION)),
        ]
        .into_iter()
        .collect()
    })
}

pub fn listen() {
    let server_address = ServerConfig::new().get_server_address();
    let listener = TcpListener::bind(&server_address).unwrap();
    let endpoints = get_endpoints();
    let pool = ThreadPool::new(4);

    println!("{CRATE_NAME} v{CRATE_VERSION} listening on http://{server_address}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream, endpoints);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, endpoints: &HashMap<String, String>) {
    let buf_reader = BufReader::new(&stream);
    if let Some(Ok(request_line)) = buf_reader.lines().next() {
        let response = match endpoints.get(&request_line) {
            Some(content) => content.clone(),
            None => Response::not_found(),
        };
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        stream.write_all(Response::not_found().as_bytes()).unwrap();
    }
}
