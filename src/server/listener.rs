use crate::{
    environment::{
        app::{CRATE_NAME, CRATE_VERSION},
        server::ServerConfig,
    },
    server::thread_pool::ThreadPool,
};
use http::{request::Request, response::Response, route::Route};
use std::{
    collections::HashMap,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    sync::OnceLock,
};

/// [`ENDPOINTS`] is a `static` [`HashMap`] with path keys and [`Route`] values
/// that is initialized once in a thread-safe manner.
static ENDPOINTS: OnceLock<HashMap<String, Route>> = OnceLock::new();
fn get_endpoints() -> &'static HashMap<String, Route> {
    ENDPOINTS.get_or_init(|| {
        crate::routes::index::get_endpoints()
            .into_iter()
            .map(|r| (r.request_pattern.to_string(), r))
            .collect()
    })
}

/// [`listen`] will listen for requests to the server and dispatch responses in
/// a thread-safe manner.
/// # Example
/// ```rust
/// use minimal_api::server::listener;
///
/// fn start_server() {
///     listener::listen();
/// }
/// ```
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

    println!("{CRATE_NAME} shutting down.");
}

/// [`handle_connection`] will respond to a server request by matching the request
/// from the provided [`TcpStream`] to a "path key" in the provided [`HashMap`].
fn handle_connection(mut stream: TcpStream, all_routes_map: &HashMap<String, Route>) {
    let mut buf_reader = BufReader::new(&stream);
    let mut request_line_str = String::new();

    // read the first line of the request (e.g., "GET /squared?number=10 HTTP/1.1")
    if buf_reader.read_line(&mut request_line_str).is_err() || request_line_str.trim().is_empty() {
        stream
            .write_all(Response::bad_request().to_string().as_bytes())
            .unwrap();
        return;
    }

    let request_line_str = request_line_str.trim();
    let parts: Vec<&str> = request_line_str.splitn(3, ' ').collect();

    if parts.len() != 3 {
        stream
            .write_all(Response::bad_request().to_string().as_bytes())
            .unwrap();
        return;
    }

    let method = parts[0].to_string(); // e.g., "GET"
    let full_path_with_query = parts[1].to_string(); // e.g., "/squared?number=10"
    let _http_version = parts[2]; // e.g., "HTTP/1.1"

    // extract base path for HashMap lookup (e.g., "/squared")
    let path_for_lookup = full_path_with_query
        .split('?')
        .next()
        .unwrap_or(&full_path_with_query)
        .to_string();

    // read headers and body
    let mut headers = HashMap::new();
    let mut content_length: usize = 0;
    let mut current_line: String = String::new();
    while buf_reader.read_line(&mut current_line).is_ok() && current_line.trim() != "" {
        if let Some((key, value)) = current_line.split_once(':') {
            headers.insert(key.trim().to_lowercase(), value.trim().to_string());
            if key.trim().to_lowercase() == "content-length" {
                content_length = value.trim().parse().unwrap_or(0);
            }
        }
        current_line.clear();
    }

    let mut body_content = String::new();
    if content_length > 0 {
        let mut body_bytes = vec![0; content_length];
        if buf_reader.read_exact(&mut body_bytes).is_ok() {
            body_content = String::from_utf8_lossy(&body_bytes).to_string();
        }
    }

    // this `Request` object is what gets passed to your route handlers.
    let incoming_request = Request {
        path: full_path_with_query,
        method: method.clone(),
        body_content: if body_content.is_empty() {
            None
        } else {
            Some(body_content)
        },
    };

    // route matching and handler invocation
    let final_response = if let Some(route) = all_routes_map.get(&path_for_lookup) {
        // route found - call its handler closure with the incoming_request
        (route.handler)(incoming_request)
    } else {
        // no route matched
        Response::not_found()
    };

    // send response back
    stream
        .write_all(final_response.to_string().as_bytes())
        .unwrap();
}
