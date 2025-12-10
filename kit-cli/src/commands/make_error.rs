use console::style;
use std::fs;
use std::path::Path;

use crate::templates;

pub fn run(name: String) {
    // Convert to snake_case for file name
    let file_name = to_snake_case(&name);

    // Convert to PascalCase for struct name
    let struct_name = to_pascal_case(&name);

    // Validate the resulting name is a valid Rust identifier
    if !is_valid_identifier(&file_name) {
        eprintln!(
            "{} '{}' is not a valid error name",
            style("Error:").red().bold(),
            name
        );
        std::process::exit(1);
    }

    let errors_dir = Path::new("src/errors");
    let error_file = errors_dir.join(format!("{}.rs", file_name));
    let mod_file = errors_dir.join("mod.rs");

    // Check if we're in a Kit project (src directory should exist)
    if !Path::new("src").exists() {
        eprintln!("{} src directory not found", style("Error:").red().bold());
        eprintln!(
            "{}",
            style("Make sure you're in a Kit project root directory.").dim()
        );
        std::process::exit(1);
    }

    // Create errors directory if it doesn't exist
    let created_dir = if !errors_dir.exists() {
        if let Err(e) = fs::create_dir_all(errors_dir) {
            eprintln!(
                "{} Failed to create errors directory: {}",
                style("Error:").red().bold(),
                e
            );
            std::process::exit(1);
        }
        println!("{} Created src/errors/", style("✓").green());
        true
    } else {
        false
    };

    // Check if error file already exists
    if error_file.exists() {
        eprintln!(
            "{} Error '{}' already exists at {}",
            style("Info:").yellow().bold(),
            struct_name,
            error_file.display()
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
                "{} Module '{}' is already declared in src/errors/mod.rs",
                style("Info:").yellow().bold(),
                file_name
            );
            std::process::exit(0);
        }
    }

    // Generate error file content
    let error_content = templates::error_template(&struct_name);

    // Write error file
    if let Err(e) = fs::write(&error_file, error_content) {
        eprintln!(
            "{} Failed to write error file: {}",
            style("Error:").red().bold(),
            e
        );
        std::process::exit(1);
    }
    println!("{} Created {}", style("✓").green(), error_file.display());

    // Update or create mod.rs
    if mod_file.exists() {
        if let Err(e) = update_mod_file(&mod_file, &file_name) {
            eprintln!(
                "{} Failed to update mod.rs: {}",
                style("Error:").red().bold(),
                e
            );
            std::process::exit(1);
        }
        println!("{} Updated src/errors/mod.rs", style("✓").green());
    } else {
        // Create mod.rs if it doesn't exist
        let mod_content = format!("pub mod {};\n", file_name);
        if let Err(e) = fs::write(&mod_file, mod_content) {
            eprintln!(
                "{} Failed to create mod.rs: {}",
                style("Error:").red().bold(),
                e
            );
            std::process::exit(1);
        }
        println!("{} Created src/errors/mod.rs", style("✓").green());
    }

    println!();
    println!(
        "Error {} created successfully!",
        style(&struct_name).cyan().bold()
    );
    println!();
    println!("Usage:");
    println!("  {} Import in your controller:", style("1.").dim());
    println!("     use crate::errors::{}::{};", file_name, struct_name);
    println!();
    println!("  {} Return as error:", style("2.").dim());
    println!("     Err({})?", struct_name);
    println!();

    // If we created the directory, remind user to add module declaration
    if created_dir {
        println!(
            "{}",
            style("Note: Make sure to add `mod errors;` to your src/main.rs").yellow()
        );
        println!();
    }
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

fn update_mod_file(mod_file: &Path, file_name: &str) -> Result<(), String> {
    let content =
        fs::read_to_string(mod_file).map_err(|e| format!("Failed to read mod.rs: {}", e))?;

    let pub_mod_decl = format!("pub mod {};", file_name);

    // Find position to insert pub mod declaration (after other pub mod declarations)
    let mut lines: Vec<&str> = content.lines().collect();

    // Find the last pub mod declaration line
    let mut last_pub_mod_idx = None;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("pub mod ") {
            last_pub_mod_idx = Some(i);
        }
    }

    // Insert pub mod declaration
    let insert_idx = match last_pub_mod_idx {
        Some(idx) => idx + 1,
        None => {
            // If no pub mod declarations, insert at the beginning (after any doc comments)
            let mut insert_idx = 0;
            for (i, line) in lines.iter().enumerate() {
                if line.starts_with("//!") || line.is_empty() {
                    insert_idx = i + 1;
                } else {
                    break;
                }
            }
            insert_idx
        }
    };
    lines.insert(insert_idx, &pub_mod_decl);

    let new_content = lines.join("\n");
    fs::write(mod_file, new_content).map_err(|e| format!("Failed to write mod.rs: {}", e))?;

    Ok(())
}
