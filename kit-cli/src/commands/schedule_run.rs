//! schedule:run command - Run all due scheduled tasks once

use console::style;
use std::process::Command;

pub fn run() {
    println!("{} Running due scheduled tasks...", style("->").cyan());
    println!();

    // Run cargo run -- schedule:run (unified binary)
    let status = Command::new("cargo")
        .args(["run", "--quiet", "--", "schedule:run"])
        .status()
        .expect("Failed to execute cargo command");

    if !status.success() {
        eprintln!();
        eprintln!("{} Schedule run failed", style("Error:").red().bold());
        std::process::exit(1);
    }
}
