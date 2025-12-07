use kit::{Config, Server};

mod config;
mod controllers;
mod routes;

#[tokio::main]
async fn main() {
    // Initialize framework configuration (loads .env files)
    Config::init(std::path::Path::new("."));

    // Register application configs
    config::register_all();

    let router = routes::register();

    // Use config-based server (reads SERVER_PORT from .env)
    Server::from_config(router)
        .run()
        .await
        .expect("Failed to start server");
}
