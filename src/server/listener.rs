use crate::{
    environment::{
        app::{CRATE_NAME, CRATE_VERSION},
        server::ServerConfig,
    },
    http::response::Response,
    server::thread_pool::ThreadPool,
};
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

pub fn listen() {
    let server_address = ServerConfig::new().get_server_address();
    let listener = TcpListener::bind(&server_address).unwrap();
    let pool = ThreadPool::new(4);

    println!("API listening on {server_address}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let contents = match &request_line[..] {
        "GET / HTTP/1.1" => Response::ok(&format!("Hello from {CRATE_NAME} v{CRATE_VERSION}!")),
        "GET /name HTTP/1.1" => Response::ok(CRATE_NAME),
        "GET /version HTTP/1.1" => Response::ok(CRATE_VERSION),
        _ => Response::not_found(),
    };

    stream.write_all(contents.as_bytes()).unwrap();
}
