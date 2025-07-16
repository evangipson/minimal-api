use crate::{
    environment::{
        app::{CRATE_NAME, CRATE_VERSION},
        server::ServerConfig,
    },
    server::thread_pool::ThreadPool,
};
use http::{request::Request, response::Response, route::Route};
use logger::{log_debug, log_info, log_warning};
use std::{
    collections::HashMap,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    sync::OnceLock,
};

/// [`ENDPOINTS`] is a `static` [`Vec`] of [`Route`] values that is initialized once
/// in a thread-safe manner.
static ENDPOINTS: OnceLock<Vec<Route>> = OnceLock::new();
fn get_endpoints() -> &'static Vec<Route> {
    ENDPOINTS.get_or_init(crate::routes::index::get_endpoints)
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
    let server_config = ServerConfig::new();
    let listener = TcpListener::bind(server_config.get_server_address()).unwrap();
    let endpoints = get_endpoints();
    let pool = ThreadPool::new(server_config.workers);

    log_info!(
        "{CRATE_NAME} v{CRATE_VERSION} listening on http://{}",
        server_config.get_server_address()
    );

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream, endpoints);
        });
    }

    log_info!("{CRATE_NAME} shutting down.");
}

/// [`handle_connection`] will respond to a server request by matching the request
/// from the provided [`TcpStream`] to a [`Route`] in the provided `all_routes_vec`.
fn handle_connection(mut stream: TcpStream, all_routes_vec: &[Route]) {
    log_debug!("handling server connection.");

    let mut buf_reader = BufReader::new(&stream);
    let mut request_line_str = String::new();

    // read the first line of the request (e.g., "GET /get/person/123?name=Alice HTTP/1.1")
    if buf_reader.read_line(&mut request_line_str).is_err() || request_line_str.trim().is_empty() {
        log_warning!("can't read request, returning 400 BAD REQUEST.");
        stream
            .write_all(Response::bad_request().to_string().as_bytes())
            .unwrap();
        return;
    }

    let request_line_str = request_line_str.trim();
    let parts: Vec<&str> = request_line_str.splitn(3, ' ').collect();

    if parts.len() != 3 {
        log_warning!("request is malformed, returning 400 BAD REQUEST.");
        stream
            .write_all(Response::bad_request().to_string().as_bytes())
            .unwrap();
        return;
    }

    let method = parts[0].to_string(); // e.g., "GET"
    let full_path_with_query = parts[1]; // e.g., "/get/person/123?name=Alice"
    let _http_version = parts[2];

    // extract base path for matching (without query string)
    let path_to_match = full_path_with_query
        .split('?')
        .next()
        .unwrap_or(full_path_with_query)
        .to_string();

    // read headers and body
    let mut headers = HashMap::new();
    let mut content_length: usize = 0;
    let mut current_line = String::new();
    while buf_reader.read_line(&mut current_line).is_ok() && current_line.trim() != "" {
        if let Some((key, value)) = current_line.split_once(':') {
            headers.insert(key.trim().to_lowercase(), value.trim().to_string());
            if key.trim().to_lowercase() == "content-length" {
                content_length = value.trim().parse().unwrap_or(0);
            }
        }
        current_line.clear();
    }

    let mut body_bytes = vec![0; content_length];
    let body_content = if content_length > 0 && buf_reader.read_exact(&mut body_bytes).is_ok() {
        Some(String::from_utf8_lossy(&body_bytes).to_string())
    } else {
        None
    };

    // iterate through ALL registered routes to find a match
    let matched_response = all_routes_vec
        .iter()
        .filter(|&route| route.method == method)
        .filter(|&route| route.matches_path(&path_to_match).is_some())
        .take(1)
        .next()
        .map(|route| {
            let incoming_request = Request {
                path: full_path_with_query.to_string(),
                method: method.clone(),
                body_content: body_content.clone(),
                path_params: route.matches_path(&path_to_match).unwrap(),
            };
            route.get_response(incoming_request)
        });

    if matched_response.is_none() {
        log_warning!("request did not match any existing routes, returning 404 NOT FOUND");
        stream
            .write_all(Response::not_found().to_string().as_bytes())
            .unwrap();
        return;
    }

    // log the routing result and send it back to the stream
    let final_response = matched_response.unwrap();
    log_info!(
        "{} {} -> {}",
        method,
        parts[1].to_string(),
        final_response.status
    );
    stream
        .write_all(final_response.to_string().as_bytes())
        .unwrap();
}
