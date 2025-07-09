# Minimal API
A thread-safe minimal API written in rust that serves JSON for HTTP requests.

Minimal API currently supports:
- `GET` HTTP requests
- `POST` HTTP requests
- Thread-safe workers to listen for requests, and serve out responses
- Query strings as parameters to functions
- [A series of more complex routes](src/routes/mock/) to represent more realistic, complex scenarios

Minimal API is planned to support:
- `PUT` HTTP requests
- `DELETE` HTTP requests

## Getting Started
1. Download the repo
1. Navigate to the repo root
1. Run the server either by:
    - `docker compose up` to launch the API in Docker
    - `cargo run` to launch the API locally
1. Use your browser to hit the API and get a JSON response from an endpoint
    - Check the [server config file](.cargo/) of the environment you chose for the address
    - By default, there are index ("/"), "/name", and "/version" endpoints

## Basic Endpoints
The following example sets up an endpoint at the index route ("/") that returns "Hello!":
```rust
use attributes::http_get;
use http::route::Route;

#[http_get("/")]
pub fn say_hello() -> String {
    format!("Hello!")
}
```

The following example sets up an endpoint at the "/who" endpoint that returns a message with the value of the `name` query parameter:
```rust
use attributes::http_get;
use http::route::Route;

#[http_get("/who")]
pub fn say_hello(name: &str) -> String {
    format!("Hello, {name}!")
}
```

## Creating Endpoints
1. Create a file in the [routes definition folder](./src/routes)
1. Write a function that returns a `String`
1. Add any [HTTP macro attribute](libs/attributes/src/lib.rs) to the function
1. Optionally, add any parameters to represent query strings or `POST` data
1. Add the function to the vector returned by [`get_endpoints()` in the routes definition file](./src/routes/index.rs), without providing any parameters if they were added
1. The [server listen() function](./src/server/listener.rs) will automatically pick up the new endpoint

## Environment Configuration
- Modify the [`./cargo/config.toml`](.cargo/config.toml) file to change the ip address or port for local development.
- Modify the [`./cargo/config.docker.toml`](.cargo/config.docker.toml) and [`./compose.yml`](compose.yml) files to change the ip address or port for docker development.