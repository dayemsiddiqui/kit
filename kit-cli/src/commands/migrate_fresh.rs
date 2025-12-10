use console::style;
use std::path::Path;
use std::process::Command;

pub fn run() {
    // Check we're in a Kit project
    if !Path::new("src/migrations").exists() {
        eprintln!(
            "{} No migrations directory found at src/migrations",
            style("Error:").red().bold()
        );
        eprintln!(
            "{}",
            style("Run 'kit make:migration <name>' to create your first migration.").dim()
        );
        std::process::exit(1);
    }

    // Check if migrate binary exists
    if !Path::new("src/bin/migrate.rs").exists() {
        eprintln!(
            "{} Migration binary not found at src/bin/migrate.rs",
            style("Error:").red().bold()
        );
        eprintln!(
            "{}",
            style("Make sure your project has the migration binary configured.").dim()
        );
        std::process::exit(1);
    }

    println!(
        "{} Dropping all tables and re-running migrations...",
        style("⚠").yellow()
    );
    println!(
        "{}",
        style("WARNING: This will delete all data in your database!").red()
    );

    // Run cargo run --bin migrate fresh
    let status = Command::new("cargo")
        .args(["run", "--bin", "migrate", "--", "fresh"])
        .status()
        .expect("Failed to execute cargo command");

    if !status.success() {
        eprintln!("{} Fresh migration failed", style("Error:").red().bold());
        std::process::exit(1);
    }
}
