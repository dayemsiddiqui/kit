//! web:run command - Run the web server

use console::style;
use std::process::Command;

pub fn run() {
    println!("{} Starting web server...", style("->").cyan());
    println!("{}", style("Press Ctrl+C to stop").dim());
    println!();

    // Run cargo run -- web:run (unified binary)
    let status = Command::new("cargo")
        .args(["run", "--quiet", "--", "web:run"])
        .status()
        .expect("Failed to execute cargo command");

    if !status.success() {
        if let Some(code) = status.code() {
            if code != 130 {
                eprintln!();
                eprintln!("{} Web server exited with error", style("Error:").red().bold());
                std::process::exit(1);
            }
        }
    }

    println!();
    println!("{} Web server stopped.", style("->").cyan());
}
