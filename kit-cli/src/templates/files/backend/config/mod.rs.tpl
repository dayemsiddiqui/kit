mod database;
mod mail;

pub use database::DatabaseConfig;
pub use mail::MailConfig;

use kit::{Config, DatabaseConfig as KitDatabaseConfig};

/// Register all application configs
pub fn register_all() {
    // Use Kit's built-in DatabaseConfig
    Config::register(KitDatabaseConfig::from_env());
    Config::register(MailConfig::from_env());
}
