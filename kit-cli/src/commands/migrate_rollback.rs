use console::style;
use std::path::Path;
use std::process::Command;

pub fn run(step: u32) {
    // Check we're in a Kit project
    if !Path::new("src/migrations").exists() {
        eprintln!(
            "{} No migrations directory found at src/migrations",
            style("Error:").red().bold()
        );
        eprintln!(
            "{}",
            style("Make sure you're in a Kit project root directory.").dim()
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
        "{} Rolling back {} migration(s)...",
        style("→").cyan(),
        step
    );

    // Run cargo run --bin migrate rollback <step>
    let status = Command::new("cargo")
        .args(["run", "--bin", "migrate", "--", "rollback", &step.to_string()])
        .status()
        .expect("Failed to execute cargo command");

    if !status.success() {
        eprintln!(
            "{} Rollback failed",
            style("Error:").red().bold()
        );
        std::process::exit(1);
    }
}
