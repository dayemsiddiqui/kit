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

    println!(
        "{} Dropping all tables and re-running migrations...",
        style("!!").yellow()
    );
    println!(
        "{}",
        style("WARNING: This will delete all data in your database!").red()
    );

    // Run cargo run -- migrate:fresh (unified binary)
    let status = Command::new("cargo")
        .args(["run", "--quiet", "--", "migrate:fresh"])
        .status()
        .expect("Failed to execute cargo command");

    if !status.success() {
        eprintln!("{} Fresh migration failed", style("Error:").red().bold());
        std::process::exit(1);
    }
}
