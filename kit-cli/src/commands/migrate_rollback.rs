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

    println!(
        "{} Rolling back {} migration(s)...",
        style("->").cyan(),
        step
    );

    // Run cargo run -- migrate:rollback <step> (unified binary)
    let status = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--",
            "migrate:rollback",
            &step.to_string(),
        ])
        .status()
        .expect("Failed to execute cargo command");

    if !status.success() {
        eprintln!("{} Rollback failed", style("Error:").red().bold());
        std::process::exit(1);
    }
}
