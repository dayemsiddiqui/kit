use kit::{Config, Server};

mod actions;
mod bootstrap;
mod config;
mod controllers;
mod middleware;
mod routes;

#[tokio::main]
async fn main() {
    // Initialize framework configuration (loads .env files)
    Config::init(std::path::Path::new("."));

    // Register application configs
    config::register_all();

    // Register services that need runtime configuration
    bootstrap::register();

    let router = routes::register();

    Server::from_config(router)
        .middleware(middleware::LoggingMiddleware) // Global middleware
        .run()
        .await
        .expect("Failed to start server");
}
