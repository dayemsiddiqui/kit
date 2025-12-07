use kit::env;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Database driver (postgres, mysql, sqlite)
    pub driver: String,
    /// Database host
    pub host: String,
    /// Database port
    pub port: u16,
    /// Database name
    pub database: String,
    /// Database username
    pub username: String,
    /// Database password
    pub password: String,
}

impl DatabaseConfig {
    /// Build config from environment variables
    pub fn from_env() -> Self {
        Self {
            driver: env("DB_DRIVER", "postgres".to_string()),
            host: env("DB_HOST", "localhost".to_string()),
            port: env("DB_PORT", 5432),
            database: env("DB_DATABASE", "kit_app".to_string()),
            username: env("DB_USERNAME", "".to_string()),
            password: env("DB_PASSWORD", "".to_string()),
        }
    }

    /// Get the connection string for this database
    pub fn connection_string(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.driver, self.username, self.password, self.host, self.port, self.database
        )
    }
}
