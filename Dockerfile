# Get the latest most lightweight rust builder image
FROM rust:1.86-alpine AS builder

# Copy over the source code
WORKDIR /usr/src/minimal-api
COPY libs ./libs
COPY src ./src
COPY Cargo.toml .

# Copy over the environment config
COPY .cargo/config.docker.toml ./.cargo/config.toml

# Get the lib linking dependency prerequistes
RUN apk add musl-dev

# Build the rust application
RUN cargo install --path .

# Get a lightweight image for the rust runtime
FROM debian:bullseye-slim

# Add any dependencies needed to run the application
RUN apt-get update && rm -rf /var/lib/apt/lists/*

# Copy the build over from the builder image
COPY --from=builder /usr/local/cargo/bin/minimal-api /usr/local/bin/minimal-api

# Run the application
CMD ["minimal-api"]