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

    // Check if migrate binary exists in Cargo.toml
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
        "{} Running migrations...",
        style("→").cyan()
    );

    // Run cargo run --bin migrate
    let status = Command::new("cargo")
        .args(["run", "--bin", "migrate", "--", "up"])
        .status()
        .expect("Failed to execute cargo command");

    if !status.success() {
        eprintln!(
            "{} Migration failed",
            style("Error:").red().bold()
        );
        std::process::exit(1);
    }
}
