# Minimal API
A thread-safe minimal API written in rust that serves JSON for HTTP requests.

## Getting Started
1. Download the repo
1. Navigate to the repo root
1. Run `docker compose up` to launch the API in Docker
1. Run `cargo run` to launch the API locally
1. Use your browser to hit the API and get a JSON response from an endpoint
    - By default, this will be http://localhost:8080
    - By default, there are index ("/"), "/name", and "/version" endpoints

## Creating Endpoints
1. Create a file in the [routes definition folder](./src/routes)
1. Add `use attributes::http_get;` to the using statements
1. Write a function that returns a `String`
1. Add the `#[http_get("/desired/endpoint")]` attribute to the function
1. Add the function to vector returned by the `get_endpoints()` function in the [routes definition file](./src/routes/index.rs)
1. The [server listen() function](./src/server/listener.rs) will automatically pick up the new `GET` endpoint

## Environment Configuration
Modify the [`./cargo/config.toml`](.cargo/config.toml) file to change the ip address or port for local development.

Modify the [`./cargo/config.docker.toml`](.cargo/config.docker.toml) and [`./compose.yml`](compose.yml) files to change the ip address or port for docker development.