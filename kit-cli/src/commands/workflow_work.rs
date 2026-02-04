//! workflow:work command - Run the workflow worker daemon

use console::style;
use std::process::Command;

pub fn run() {
    println!("{} Starting workflow worker...", style("->").cyan());
    println!("{}", style("Press Ctrl+C to stop").dim());
    println!();

    let status = Command::new("cargo")
        .args(["run", "--quiet", "--", "workflow:work"])
        .status()
        .expect("Failed to execute cargo command");

    if !status.success() {
        if let Some(code) = status.code() {
            if code != 130 {
                eprintln!();
                eprintln!(
                    "{} Workflow worker exited with error (code: {})",
                    style("Error:").red().bold(),
                    code
                );
                std::process::exit(1);
            }
        }
    }

    println!();
    println!("{} Workflow worker stopped.", style("->").cyan());
}
