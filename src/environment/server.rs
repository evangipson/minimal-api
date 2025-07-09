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
}

impl ServerConfig {
    pub fn new() -> Self {
        let ip_address = env!("IP_ADDR").to_string();
        let port = env!("PORT").to_string();

        if ip_address.is_empty() {
            panic!("IP_ADDR not defined in .cargo/config.toml.");
        } else if port.is_empty() {
            panic!("PORT not defined in .cargo/config.toml.");
        }

        ServerConfig { ip_address, port }
    }

    pub fn get_server_address(&self) -> String {
        format!("{}:{}", self.ip_address, self.port)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig::new()
    }
}
