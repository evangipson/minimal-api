use std::env;

pub struct ServerConfig {
    pub ip_address: String,
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
