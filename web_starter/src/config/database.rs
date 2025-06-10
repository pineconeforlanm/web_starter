use num_cpus;
use serde::Deserialize;
use std::sync::LazyLock;

const DEFAULT_DB_HOST: &str = "localhost";
const DEFAULT_DB_PORT: u16 = 5432;
const DEFAULT_DB_USER: &str = "postgres";
const DEFAULT_DB_PASSWORD: &str = "postgres";
const DEFAULT_DB_DATABASE: &str = "postgres";
const DEFAULT_DB_SCHEMA: &str = "public";

static DEFAULT_MAX_CONNECTIONS: LazyLock<u32> = LazyLock::new(|| {
    let cpus = num_cpus::get() as u32;
    cpus * 8
});
static DEFAULT_MIN_CONNECTIONS: LazyLock<u32> = LazyLock::new(|| {
    let cpus = num_cpus::get() as u32;
    cpus
});

const DEFAULT_CONNECT_TIMEOUT: u64 = 10; // seconds
const DEFAULT_ACQUIRE_TIMEOUT: u64 = 30; // seconds
const DEFAULT_IDLE_TIMEOUT: u64 = 300; // seconds
const DEFAULT_MAX_LIFETIME: u64 = 3600 * 24; // seconds

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<String>,
    database: Option<String>,
    schema: Option<String>,
    max_connections: Option<u32>,
    min_connections: Option<u32>,
    connect_timeout: Option<u64>,
    acquire_timeout: Option<u64>,   
    idle_timeout: Option<u64>,
    max_lifetime: Option<u64>,
    sqlx_logging: Option<bool>,
}

impl DatabaseConfig {
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or(DEFAULT_DB_HOST)
    }
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(DEFAULT_DB_PORT)
    }
    pub fn user(&self) -> &str {
        self.user.as_deref().unwrap_or(DEFAULT_DB_USER)
    }
    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or(DEFAULT_DB_PASSWORD)
    }
    pub fn database(&self) -> &str {
        self.database.as_deref().unwrap_or(DEFAULT_DB_DATABASE)
    }
    pub fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or(DEFAULT_DB_SCHEMA)
    }
    pub fn max_connections(&self) -> u32 {
        self.max_connections.unwrap_or(*DEFAULT_MAX_CONNECTIONS)
    }
    pub fn min_connections(&self) -> u32 {
        self.min_connections.unwrap_or(*DEFAULT_MIN_CONNECTIONS)
    }
    pub fn connect_timeout(&self) -> u64 {
        self.connect_timeout.unwrap_or(DEFAULT_CONNECT_TIMEOUT)
    }
    pub fn acquire_timeout(&self) -> u64 {
        self.acquire_timeout.unwrap_or(DEFAULT_ACQUIRE_TIMEOUT)
    }
    pub fn idle_timeout(&self) -> u64 {
        self.idle_timeout.unwrap_or(DEFAULT_IDLE_TIMEOUT)
    }
    pub fn max_lifetime(&self) -> u64 {
        self.max_lifetime.unwrap_or(DEFAULT_MAX_LIFETIME)
    }
    pub fn sqlx_logging(&self) -> bool {
        self.sqlx_logging.unwrap_or(false)  
    }
}
