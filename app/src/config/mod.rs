mod database;
mod mail;

pub use database::DatabaseConfig;
pub use mail::MailConfig;

use kit::Config;

/// Register all application configs
pub fn register_all() {
    Config::register(DatabaseConfig::from_env());
    Config::register(MailConfig::from_env());
}
