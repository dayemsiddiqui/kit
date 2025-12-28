//! Kit Application Entry Point

use kit::Application;
use sea_orm_migration::prelude::MigratorTrait;

mod actions;
mod bootstrap;
mod config;
mod controllers;
mod middleware;
mod migrations;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    Application::new()
        .config(config::register_all)
        .bootstrap(bootstrap::register)
        .routes(routes::register)
        .migrations::<migrations::Migrator>()
        .run()
        .await;
}
