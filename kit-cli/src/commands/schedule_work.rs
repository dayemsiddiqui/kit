//! schedule:work command - Run the scheduler daemon

use console::style;
use std::process::Command;

pub fn run() {
    println!("{} Starting scheduler daemon...", style("->").cyan());
    println!(
        "{}",
        style("Press Ctrl+C to stop").dim()
    );
    println!();

    // Run cargo run -- schedule:work (unified binary)
    let status = Command::new("cargo")
        .args(["run", "--quiet", "--", "schedule:work"])
        .status()
        .expect("Failed to execute cargo command");

    if !status.success() {
        // Exit code might be from Ctrl+C, which is expected
        // Only print error if it wasn't interrupted
        if let Some(code) = status.code() {
            if code != 130 {
                // 130 = interrupted by Ctrl+C
                eprintln!();
                eprintln!(
                    "{} Scheduler daemon exited with error (code: {})",
                    style("Error:").red().bold(),
                    code
                );
                std::process::exit(1);
            }
        }
    }

    println!();
    println!("{} Scheduler daemon stopped.", style("->").cyan());
}
