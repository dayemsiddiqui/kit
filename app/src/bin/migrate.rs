//! Migration runner binary
//!
//! This binary handles database migrations using SeaORM.
//! Usage:
//!   cargo run --bin migrate           - Run all pending migrations
//!   cargo run --bin migrate rollback  - Rollback the last migration
//!   cargo run --bin migrate status    - Show migration status
//!   cargo run --bin migrate fresh     - Drop all tables and re-run migrations

use sea_orm_migration::prelude::*;
use std::env;
use std::path::Path;

// Include the migrations module from the main crate
#[path = "../migrations/mod.rs"]
mod migrations;

use migrations::Migrator;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // For SQLite, ensure the database file can be created
    let database_url = if database_url.starts_with("sqlite://") {
        // Extract the file path from the URL
        let path = database_url.trim_start_matches("sqlite://");
        let path = path.trim_start_matches("./");

        // Create parent directories if needed
        if let Some(parent) = Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).ok();
            }
        }

        // Touch the file to create it if it doesn't exist
        if !Path::new(path).exists() {
            std::fs::File::create(path).ok();
        }

        // Use the file path format that SQLite prefers
        format!("sqlite:{}?mode=rwc", path)
    } else {
        database_url
    };

    let db = sea_orm::Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("up");

    match command {
        "up" | "migrate" => {
            println!("Running migrations...");
            Migrator::up(&db, None).await.expect("Failed to run migrations");
            println!("Migrations completed successfully!");
        }
        "down" | "rollback" => {
            let steps: u32 = args
                .get(2)
                .and_then(|s| s.parse().ok())
                .unwrap_or(1);
            println!("Rolling back {} migration(s)...", steps);
            Migrator::down(&db, Some(steps)).await.expect("Failed to rollback");
            println!("Rollback completed successfully!");
        }
        "status" => {
            println!("Migration status:");
            Migrator::status(&db).await.expect("Failed to get status");
        }
        "fresh" => {
            println!("WARNING: Dropping all tables and re-running migrations...");
            Migrator::fresh(&db).await.expect("Failed to refresh database");
            println!("Database refreshed successfully!");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Usage: migrate [up|rollback|status|fresh]");
            std::process::exit(1);
        }
    }
}
