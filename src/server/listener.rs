use crate::{
    environment::{
        app::{CRATE_NAME, CRATE_VERSION},
        server::ServerConfig,
    },
    server::thread_pool::ThreadPool,
};
use http::response::Response;
use std::{
    collections::HashMap,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    sync::OnceLock,
};

// use OnceLock to only initialize the static collection of endpoints once in a thread-safe manner
static ENDPOINTS: OnceLock<HashMap<String, Response>> = OnceLock::new();
fn get_endpoints() -> &'static HashMap<String, Response> {
    ENDPOINTS.get_or_init(|| {
        crate::routes::index::get_endpoints()
            .into_iter()
            .map(|r| (r.request.to_string(), r.get_primary_route()))
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

fn handle_connection(mut stream: TcpStream, endpoints: &HashMap<String, Response>) {
    let buf_reader = BufReader::new(&stream);
    if let Some(Ok(request_line)) = buf_reader.lines().next() {
        let response = match endpoints.get(&request_line) {
            Some(content) => content,
            None => &Response::not_found(),
        };
        stream.write_all(response.to_string().as_bytes()).unwrap();
    } else {
        stream
            .write_all(Response::not_found().to_string().as_bytes())
            .unwrap();
    }
}
