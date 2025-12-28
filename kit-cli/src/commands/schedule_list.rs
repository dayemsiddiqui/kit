//! schedule:list command - Display all registered scheduled tasks

use console::style;
use std::process::Command;

pub fn run() {
    // Run cargo run -- schedule:list (unified binary)
    let status = Command::new("cargo")
        .args(["run", "--quiet", "--", "schedule:list"])
        .status()
        .expect("Failed to execute cargo command");

    if !status.success() {
        eprintln!();
        eprintln!(
            "{} Failed to list scheduled tasks",
            style("Error:").red().bold()
        );
        std::process::exit(1);
    }
}
