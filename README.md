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

## Environment Configuration
Modify the [`./cargo/config.toml`](.cargo/config.toml) file to change the ip address or port for local development.

Modify the [`./cargo/config.docker.toml`](.cargo/config.docker.toml) and [`./compose.yml`](compose.yml) files to change the ip address or port for docker development.