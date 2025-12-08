mod actions;
mod bootstrap;
mod config;
mod controllers;
mod middleware;
mod routes;

use kit::{Config, Server};

#[tokio::main]
async fn main() {
    // Initialize framework configuration (loads .env files)
    Config::init(std::path::Path::new("."));

    // Register application configs
    config::register_all();

    // Register services and global middleware
    bootstrap::register().await;

    let router = routes::register();

    // Create server with configuration from environment
    Server::from_config(router)
        .run()
        .await
        .expect("Failed to start server");
}
