use std::env;

/// [`CRATE_NAME`] represents the name of the crate, set with the
/// `package::name` value in the root-level `Cargo.toml` file.
pub const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

/// [`CRATE_VERSION`] represents the version of the crate, set with
/// the `package::version` value in the root-level `Cargo.toml` file.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
