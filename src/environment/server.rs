use std::env;

/// [`ServerConfig`] represents a set of environmental server configurations.
pub struct ServerConfig {
    /// [`ServerConfig::ip_address`] is the environmental ip address of the server,
    /// which is set by the `IP_ADDR` value in the config files in the `.cargo`
    /// directory.
    pub ip_address: String,
    /// [`ServerConfig::port`] is the environmental port that the server will listen
    /// on, which is set by the `PORT` value in the config files in the `.cargo`
    /// directory.
    pub port: String,
    /// [`ServerConfig::workers`] is the number of workers that the server has to
    /// handle requests (essentially individual threads dedicated to the server),
    /// which is set by the `WORKERS` value in the config files in the `.cargo`
    /// directory.
    pub workers: usize,
}

impl ServerConfig {
    /// [`ServerConfig::new`] will create a [`ServerConfig`] which reads values from
    /// the configuration files in the `.cargo` directory.
    ///
    /// # Example
    /// [`ServerConfig::new`] can be used to create a new [`ServerConfig`]:
    /// ```rust
    /// use minimal_api::environment::server::ServerConfig;
    ///
    /// fn create_server_config() -> ServerConfig {
    ///     ServerConfig::new()
    /// }
    /// ```
    pub fn new() -> Self {
        let ip_address = env!("IP_ADDR").to_string();
        let port = env!("PORT").to_string();
        let workers = env!("WORKERS")
            .parse::<usize>()
            .expect("cannot parse WORKERS defined in .cargo/config.toml, please check the value.");

        if ip_address.is_empty() {
            panic!("IP_ADDR not defined in .cargo/config.toml.");
        } else if port.is_empty() {
            panic!("PORT not defined in .cargo/config.toml.");
        } else if workers == 0 {
            panic!("WORKERS not defined in .cargo/config.toml.");
        }

        ServerConfig {
            ip_address,
            port,
            workers,
        }
    }

    /// [`ServerConfig::get_server_address`] will get [`ServerConfig::ip_address`] and
    /// [`ServerConfig::port`], formatted with a colon between them.
    ///
    /// # Example
    /// [`ServerConfig::get_server_address`] can be used to get a [`ServerConfig`] address:
    /// ```rust
    /// use minimal_api::environment::server::ServerConfig;
    ///
    /// fn get_address(server_config: ServerConfig) -> String {
    ///     server_config.get_server_address()
    /// }
    /// ```
    pub fn get_server_address(&self) -> String {
        format!("{}:{}", self.ip_address, self.port)
    }
}

/// Implement [`Default`] for [`ServerConfig`].
impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig::new()
    }
}
