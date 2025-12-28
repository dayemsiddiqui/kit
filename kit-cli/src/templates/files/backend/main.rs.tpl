//! Kit Application Entry Point
//!
//! This is the unified entry point for the Kit application.
//! It provides subcommands for running the web server, migrations, and scheduler.
//!
//! # Usage
//!
//! ```bash
//! ./app                    # Run web server with auto-migrate (default)
//! ./app serve              # Run web server with auto-migrate
//! ./app serve --no-migrate # Run web server without auto-migrate
//! ./app migrate            # Run pending migrations
//! ./app migrate:status     # Show migration status
//! ./app migrate:rollback   # Rollback last migration
//! ./app migrate:fresh      # Drop all tables and re-run migrations
//! ./app schedule:work      # Run scheduler daemon
//! ./app schedule:run       # Run due tasks once
//! ./app schedule:list      # List registered tasks
//! ```

use clap::{Parser, Subcommand};
use kit::{Config, Server};
use sea_orm_migration::prelude::*;
use std::env;
use std::path::Path;

mod actions;
mod bootstrap;
mod config;
mod controllers;
mod middleware;
mod migrations;
mod models;
mod routes;

use migrations::Migrator;

#[derive(Parser)]
#[command(name = "app")]
#[command(about = "Kit application server and utilities")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the web server (default command)
    Serve {
        /// Skip running migrations on startup
        #[arg(long)]
        no_migrate: bool,
    },
    /// Run pending database migrations
    Migrate,
    /// Show migration status
    #[command(name = "migrate:status")]
    MigrateStatus,
    /// Rollback the last migration(s)
    #[command(name = "migrate:rollback")]
    MigrateRollback {
        /// Number of migrations to rollback
        #[arg(default_value = "1")]
        steps: u32,
    },
    /// Drop all tables and re-run all migrations
    #[command(name = "migrate:fresh")]
    MigrateFresh,
    /// Run the scheduler daemon (checks every minute)
    #[command(name = "schedule:work")]
    ScheduleWork,
    /// Run all due scheduled tasks once
    #[command(name = "schedule:run")]
    ScheduleRun,
    /// List all registered scheduled tasks
    #[command(name = "schedule:list")]
    ScheduleList,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize framework configuration (loads .env files)
    Config::init(Path::new("."));

    // Register application configs
    config::register_all();

    match cli.command {
        None | Some(Commands::Serve { no_migrate: false }) => {
            // Default: run server with auto-migrate
            run_migrations_silent().await;
            run_server().await;
        }
        Some(Commands::Serve { no_migrate: true }) => {
            // Run server without migrations
            run_server().await;
        }
        Some(Commands::Migrate) => {
            run_migrations().await;
        }
        Some(Commands::MigrateStatus) => {
            show_migration_status().await;
        }
        Some(Commands::MigrateRollback { steps }) => {
            rollback_migrations(steps).await;
        }
        Some(Commands::MigrateFresh) => {
            fresh_migrations().await;
        }
        Some(Commands::ScheduleWork) => {
            run_scheduler_daemon().await;
        }
        Some(Commands::ScheduleRun) => {
            run_scheduled_tasks().await;
        }
        Some(Commands::ScheduleList) => {
            list_scheduled_tasks().await;
        }
    }
}

async fn run_server() {
    // Register services and global middleware
    bootstrap::register().await;

    let router = routes::register();

    // Create server with configuration from environment
    Server::from_config(router)
        .run()
        .await
        .expect("Failed to start server");
}

async fn get_database_connection() -> sea_orm::DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // For SQLite, ensure the database file can be created
    let database_url = if database_url.starts_with("sqlite://") {
        let path = database_url.trim_start_matches("sqlite://");
        let path = path.trim_start_matches("./");

        if let Some(parent) = Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).ok();
            }
        }

        if !Path::new(path).exists() {
            std::fs::File::create(path).ok();
        }

        format!("sqlite:{}?mode=rwc", path)
    } else {
        database_url
    };

    sea_orm::Database::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

async fn run_migrations_silent() {
    let db = get_database_connection().await;
    if let Err(e) = Migrator::up(&db, None).await {
        eprintln!("Warning: Migration failed: {}", e);
    }
}

async fn run_migrations() {
    println!("Running migrations...");
    let db = get_database_connection().await;
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    println!("Migrations completed successfully!");
}

async fn show_migration_status() {
    println!("Migration status:");
    let db = get_database_connection().await;
    Migrator::status(&db)
        .await
        .expect("Failed to get migration status");
}

async fn rollback_migrations(steps: u32) {
    println!("Rolling back {} migration(s)...", steps);
    let db = get_database_connection().await;
    Migrator::down(&db, Some(steps))
        .await
        .expect("Failed to rollback migrations");
    println!("Rollback completed successfully!");
}

async fn fresh_migrations() {
    println!("WARNING: Dropping all tables and re-running migrations...");
    let db = get_database_connection().await;
    Migrator::fresh(&db)
        .await
        .expect("Failed to refresh database");
    println!("Database refreshed successfully!");
}

// Schedule functionality - these will work once the user creates tasks with `kit make:task`
async fn run_scheduler_daemon() {
    // Bootstrap the application for scheduler context
    bootstrap::register().await;

    println!("==============================================");
    println!("  Kit Scheduler Daemon");
    println!("==============================================");
    println!();
    println!("  Note: Create tasks with `kit make:task <name>`");
    println!("  Press Ctrl+C to stop");
    println!();
    println!("==============================================");

    // Placeholder - will be enhanced when schedule module exists
    eprintln!("Scheduler daemon is not yet configured.");
    eprintln!("Create a scheduled task with: kit make:task <name>");
    eprintln!("Then register it in src/schedule.rs");
}

async fn run_scheduled_tasks() {
    // Bootstrap the application for scheduler context
    bootstrap::register().await;

    println!("Running scheduled tasks...");
    eprintln!("Scheduler is not yet configured.");
    eprintln!("Create a scheduled task with: kit make:task <name>");
}

async fn list_scheduled_tasks() {
    println!("Registered scheduled tasks:");
    println!();
    eprintln!("No scheduled tasks registered.");
    eprintln!("Create a scheduled task with: kit make:task <name>");
}
