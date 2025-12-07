mod config;
mod controllers;
mod routes;

use kit::{Config, Server};

#[tokio::main]
async fn main() {
    // Initialize framework configuration (loads .env files)
    Config::init(std::path::Path::new("."));

    // Register application configs
    config::register_all();

    let router = routes::register();

    // Create server with configuration from environment
    Server::from_config(router)
        .run()
        .await
        .expect("Failed to start server");
}
