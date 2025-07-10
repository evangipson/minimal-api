# Minimal API
A thread-safe minimal API written in rust that serves JSON for HTTP requests.

Minimal API currently supports:
- Thread-safe workers to listen for requests, and serve out responses
- `GET`, `POST`, `PUT`, and `DELETE` HTTP requests
- Query string keys and body content as function parameters
- [A series of more complex routes](src/routes/mock/) to represent more realistic, complex scenarios

## TODO:
- [ ] Guard query string parameter (and body content) input casting with some sort of validation in the attributes library
- [ ] Introduce dynamic routes, e.g.: /get/person/{id}
- [ ] Write unit tests for core functionality

## Getting Started
1. Download the repo
1. Navigate to the repo root
1. Run the server either by:
    - `docker compose up` to launch the API in Docker
    - `cargo run` to launch the API locally
1. Use your browser to hit the API and get a JSON response from an endpoint
    - Check the [server config file](.cargo/) of the environment you chose for the address
    - By default, there are index ("/"), "/name", and "/version" endpoints

## Example routes
The following example sets up a `GET` endpoint for the index route (`/`) that returns "Hello!":
```rust
use http_attributes::http_get;
use http::route::Route;

#[http_get("/")]
pub fn say_hello() -> String {
    format!("Hello!")
}
```

The following example sets up a `GET` endpoint for the `/who` path that returns a message with the value of the `name` query parameter:
```rust
use http_attributes::http_get;
use http::route::Route;

#[http_get("/who")]
pub fn say_hello(name: String) -> String {
    format!("Hello, {name}!")
}
```

The following example sets up a `POST` endpoint for the `/submit` path that returns the `POST` data:
```rust
use http_attributes::http_post;
use http::route::Route;

#[http_post("/submit")]
pub fn get_post_data(content: String) -> String {
    format!("Received '{content}' from POST")
}
```

The following example sets up a `PUT` endpoint for the `/update` path that returns the `PUT` data:
```rust
use http_attributes::http_put;
use http::route::Route;

#[http_put("/update")]
pub fn get_put_data(content: String) -> String {
    format!("Received '{content}' from PUT")
}
```

The following example sets up a `DELETE` endpoint for the `/remove` path that returns a query parameter value sent to the `DELETE` route:
```rust
use http_attributes::http_delete;
use http::route::Route;

#[http_delete("/remove")]
pub fn get_delete_id(id: String) -> String {
    format!("Received '{id}' from DELETE")
}
```

## Creating Endpoints
1. Create a file in the [routes definition folder](./src/routes)
1. Write a function that returns a `String`
1. Add any [HTTP macro attribute](libs/attributes/src/lib.rs) to the function
1. Optionally, add any parameters to represent query strings or body data
1. Add the function to the vector returned by [`get_endpoints()` in the routes definition file](./src/routes/index.rs), without providing any parameters if they were added
1. The [server listen() function](./src/server/listener.rs) will automatically pick up the new endpoint

## Environment Configuration
- Modify the [`./cargo/config.toml`](.cargo/config.toml) file to change the ip address or port for local development.
- Modify the [`./cargo/config.docker.toml`](.cargo/config.docker.toml) and [`./compose.yml`](compose.yml) files to change the ip address or port for docker development.