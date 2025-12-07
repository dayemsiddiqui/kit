use kit::{json_response, Config, Request, Response};

use crate::config::{DatabaseConfig, MailConfig};

/// Example endpoint showing how to use config values
pub async fn show(_req: Request) -> Response {
    let db = Config::get::<DatabaseConfig>().unwrap();
    let mail = Config::get::<MailConfig>().unwrap();

    json_response!({
        "message": "Config values loaded from .env",
        "database": {
            "driver": db.driver,
            "host": db.host,
            "port": db.port,
            "database": db.database,
            "connection_string": db.connection_string()
        },
        "mail": {
            "driver": mail.driver,
            "host": mail.host,
            "port": mail.port,
            "from_address": mail.from_address,
            "from_name": mail.from_name
        }
    })
}
