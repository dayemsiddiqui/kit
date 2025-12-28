//! make:task command - Generate a new scheduled task

use console::style;
use std::fs;
use std::path::Path;

use crate::templates;

pub fn run(name: String) {
    // Convert to PascalCase for struct name
    let struct_name = to_pascal_case(&name);

    // Append "Task" suffix if not already present
    let struct_name = if struct_name.ends_with("Task") {
        struct_name
    } else {
        format!("{}Task", struct_name)
    };

    // Convert to snake_case for file name
    let file_name = to_snake_case(&struct_name);

    // Validate the resulting name is a valid Rust identifier
    if !is_valid_identifier(&file_name) {
        eprintln!(
            "{} '{}' is not a valid task name",
            style("Error:").red().bold(),
            name
        );
        std::process::exit(1);
    }

    let tasks_dir = Path::new("src/tasks");
    let task_file = tasks_dir.join(format!("{}.rs", file_name));
    let mod_file = tasks_dir.join("mod.rs");
    let schedule_file = Path::new("src/schedule.rs");

    // Ensure we're in a Kit project (check for src directory)
    if !Path::new("src").exists() {
        eprintln!(
            "{} Not in a Kit project root directory",
            style("Error:").red().bold()
        );
        eprintln!(
            "{}",
            style("Make sure you're in a Kit project directory with a src/ folder.").dim()
        );
        std::process::exit(1);
    }

    // Create tasks directory if it doesn't exist
    if !tasks_dir.exists() {
        if let Err(e) = fs::create_dir_all(tasks_dir) {
            eprintln!(
                "{} Failed to create tasks directory: {}",
                style("Error:").red().bold(),
                e
            );
            std::process::exit(1);
        }
        println!("{} Created src/tasks/", style("✓").green());

        // Create mod.rs
        let mod_content = templates::tasks_mod();
        if let Err(e) = fs::write(&mod_file, mod_content) {
            eprintln!(
                "{} Failed to create mod.rs: {}",
                style("Error:").red().bold(),
                e
            );
            std::process::exit(1);
        }
        println!("{} Created src/tasks/mod.rs", style("✓").green());
    }

    // Create schedule.rs if it doesn't exist
    if !schedule_file.exists() {
        let schedule_content = templates::schedule_rs();
        if let Err(e) = fs::write(schedule_file, schedule_content) {
            eprintln!(
                "{} Failed to create schedule.rs: {}",
                style("Error:").red().bold(),
                e
            );
            std::process::exit(1);
        }
        println!("{} Created src/schedule.rs", style("✓").green());
    }

    // Note: Scheduler is now integrated into the main binary
    // Run with: ./app schedule:work

    // Check if task file already exists
    if task_file.exists() {
        eprintln!(
            "{} Task '{}' already exists at {}",
            style("Info:").yellow().bold(),
            struct_name,
            task_file.display()
        );
        std::process::exit(0);
    }

    // Check if module is already declared in mod.rs
    if mod_file.exists() {
        let mod_content = fs::read_to_string(&mod_file).unwrap_or_default();
        let mod_decl = format!("mod {};", file_name);
        let pub_mod_decl = format!("pub mod {};", file_name);
        if mod_content.contains(&mod_decl) || mod_content.contains(&pub_mod_decl) {
            eprintln!(
                "{} Module '{}' is already declared in src/tasks/mod.rs",
                style("Info:").yellow().bold(),
                file_name
            );
            std::process::exit(0);
        }
    }

    // Generate task file content
    let task_content = templates::task_template(&file_name, &struct_name);

    // Write task file
    if let Err(e) = fs::write(&task_file, task_content) {
        eprintln!(
            "{} Failed to write task file: {}",
            style("Error:").red().bold(),
            e
        );
        std::process::exit(1);
    }
    println!("{} Created {}", style("✓").green(), task_file.display());

    // Update mod.rs
    if let Err(e) = update_mod_file(&mod_file, &file_name, &struct_name) {
        eprintln!(
            "{} Failed to update mod.rs: {}",
            style("Error:").red().bold(),
            e
        );
        std::process::exit(1);
    }
    println!("{} Updated src/tasks/mod.rs", style("✓").green());

    println!();
    println!(
        "Task {} created successfully!",
        style(&struct_name).cyan().bold()
    );
    println!();
    println!("Next steps:");
    println!(
        "  {} Implement your task logic in {}",
        style("1.").dim(),
        task_file.display()
    );
    println!();
    println!(
        "  {} Register the task in src/schedule.rs:",
        style("2.").dim()
    );
    println!(
        "     {}",
        style(format!("use crate::tasks::{};", file_name)).cyan()
    );
    println!(
        "     {}",
        style(format!("schedule.task({}::new());", struct_name)).cyan()
    );
    println!();
    println!(
        "  {} Run the scheduler:",
        style("3.").dim()
    );
    println!("     kit schedule:work  {} Daemon mode", style("#").dim());
    println!("     kit schedule:run   {} Run once", style("#").dim());
    println!("     kit schedule:list  {} List tasks", style("#").dim());
    println!();
}

fn is_valid_identifier(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut chars = name.chars();

    // First character must be letter or underscore
    match chars.next() {
        Some(c) if c.is_alphabetic() || c == '_' => {}
        _ => return false,
    }

    // Rest must be alphanumeric or underscore
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

fn update_mod_file(mod_file: &Path, file_name: &str, struct_name: &str) -> Result<(), String> {
    let content =
        fs::read_to_string(mod_file).map_err(|e| format!("Failed to read mod.rs: {}", e))?;

    let pub_mod_decl = format!("pub mod {};", file_name);
    let pub_use_decl = format!("pub use {}::{};", file_name, struct_name);

    // Find position to insert declarations
    let lines: Vec<&str> = content.lines().collect();

    // Find the last pub mod declaration line
    let mut last_pub_mod_idx = None;
    let mut last_pub_use_idx = None;

    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("pub mod ") {
            last_pub_mod_idx = Some(i);
        }
        if line.trim().starts_with("pub use ") {
            last_pub_use_idx = Some(i);
        }
    }

    // Build new content
    let mut new_lines: Vec<String> = Vec::new();

    // If we found existing pub mod declarations, insert after them
    if let Some(idx) = last_pub_mod_idx {
        for (i, line) in lines.iter().enumerate() {
            new_lines.push(line.to_string());
            if i == idx {
                new_lines.push(pub_mod_decl.clone());
            }
        }
    } else {
        // No existing pub mod declarations, add at the end (before empty lines)
        let mut content_end = lines.len();
        while content_end > 0 && lines[content_end - 1].trim().is_empty() {
            content_end -= 1;
        }

        for (i, line) in lines.iter().enumerate() {
            new_lines.push(line.to_string());
            if i == content_end.saturating_sub(1) || (content_end == 0 && i == 0) {
                new_lines.push(pub_mod_decl.clone());
            }
        }

        // If file was empty
        if lines.is_empty() {
            new_lines.push(pub_mod_decl.clone());
        }
    }

    // Now add pub use declaration if there are existing pub use declarations
    if last_pub_use_idx.is_some() {
        // Find the new position of the last pub use after our modification
        let mut insert_idx = None;
        for (i, line) in new_lines.iter().enumerate() {
            if line.trim().starts_with("pub use ") {
                insert_idx = Some(i);
            }
        }
        if let Some(idx) = insert_idx {
            new_lines.insert(idx + 1, pub_use_decl);
        }
    }

    let new_content = new_lines.join("\n");

    // Ensure file ends with newline
    let new_content = if new_content.ends_with('\n') {
        new_content
    } else {
        format!("{}\n", new_content)
    };

    fs::write(mod_file, new_content).map_err(|e| format!("Failed to write mod.rs: {}", e))?;

    Ok(())
}
