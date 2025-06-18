use serde::Deserialize;

const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: u16 = 3000;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    host: Option<String>,
    port: Option<u16>,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(DEFAULT_PORT)
    }
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or(DEFAULT_HOST)
    }
}
