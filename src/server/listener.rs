use crate::{http::response::Response, server::thread_pool::ThreadPool};
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

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
        "GET / HTTP/1.1" => Response::ok(&format!("Hello from {}!", env!("CARGO_CRATE_NAME"))),
        "GET /version HTTP/1.1" => Response::ok(env!("CARGO_PKG_VERSION")),
        _ => Response::not_found(),
    };

    stream.write_all(contents.as_bytes()).unwrap();
}
