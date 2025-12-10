use console::style;
use std::fs;
use std::path::Path;

use crate::templates;

pub fn run(name: String) {
    // Validate name is a valid Rust identifier
    if !is_valid_identifier(&name) {
        eprintln!(
            "{} '{}' is not a valid Rust identifier",
            style("Error:").red().bold(),
            name
        );
        std::process::exit(1);
    }

    // Convert name to struct name and file name
    // e.g., "Auth" -> "AuthMiddleware", "auth"
    // e.g., "RateLimit" -> "RateLimitMiddleware", "rate_limit"
    let struct_name = if name.ends_with("Middleware") {
        name.clone()
    } else {
        format!("{}Middleware", name)
    };
    let file_name = to_snake_case(&name.trim_end_matches("Middleware"));

    let middleware_dir = Path::new("src/middleware");
    let middleware_file = middleware_dir.join(format!("{}.rs", file_name));
    let mod_file = middleware_dir.join("mod.rs");

    // Check if middleware directory exists
    if !middleware_dir.exists() {
        if let Err(e) = fs::create_dir_all(middleware_dir) {
            eprintln!(
                "{} Failed to create middleware directory: {}",
                style("Error:").red().bold(),
                e
            );
            std::process::exit(1);
        }
        println!("{} Created src/middleware directory", style("✓").green());
    }

    // Check if middleware file already exists
    if middleware_file.exists() {
        eprintln!(
            "{} Middleware '{}' already exists at {}",
            style("Error:").red().bold(),
            struct_name,
            middleware_file.display()
        );
        std::process::exit(1);
    }

    // Generate middleware file content
    let middleware_content = templates::middleware_template(&name, &struct_name);

    // Write middleware file
    if let Err(e) = fs::write(&middleware_file, middleware_content) {
        eprintln!(
            "{} Failed to write middleware file: {}",
            style("Error:").red().bold(),
            e
        );
        std::process::exit(1);
    }
    println!(
        "{} Created {}",
        style("✓").green(),
        middleware_file.display()
    );

    // Update mod.rs
    if mod_file.exists() {
        if let Err(e) = update_mod_file(&mod_file, &file_name, &struct_name) {
            eprintln!(
                "{} Failed to update mod.rs: {}",
                style("Error:").red().bold(),
                e
            );
            std::process::exit(1);
        }
        println!("{} Updated src/middleware/mod.rs", style("✓").green());
    } else {
        // Create mod.rs if it doesn't exist
        let mod_content = format!(
            "//! Application middleware\n\nmod {};\n\npub use {}::{};\n",
            file_name, file_name, struct_name
        );
        if let Err(e) = fs::write(&mod_file, mod_content) {
            eprintln!(
                "{} Failed to create mod.rs: {}",
                style("Error:").red().bold(),
                e
            );
            std::process::exit(1);
        }
        println!("{} Created src/middleware/mod.rs", style("✓").green());
    }

    println!();
    println!(
        "Middleware {} created successfully!",
        style(&struct_name).cyan().bold()
    );
    println!();
    println!("Usage:");
    println!("  {} Import and use in routes:", style("1.").dim());
    println!("     use crate::middleware::{};", struct_name);
    println!("     .get(\"/path\", handler).middleware({})", struct_name);
    println!();
    println!("  {} Or apply globally in main.rs:", style("2.").dim());
    println!("     .middleware(middleware::{})", struct_name);
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

fn update_mod_file(mod_file: &Path, file_name: &str, struct_name: &str) -> Result<(), String> {
    let content =
        fs::read_to_string(mod_file).map_err(|e| format!("Failed to read mod.rs: {}", e))?;

    // Check if module already declared
    let mod_decl = format!("mod {};", file_name);
    if content.contains(&mod_decl) {
        return Err(format!("Module '{}' already declared in mod.rs", file_name));
    }

    // Find position to insert mod declaration (after other mod declarations)
    let mut lines: Vec<&str> = content.lines().collect();

    // Find the last mod declaration line
    let mut last_mod_idx = None;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("mod ") {
            last_mod_idx = Some(i);
        }
    }

    // Insert mod declaration
    let mod_insert_idx = match last_mod_idx {
        Some(idx) => idx + 1,
        None => {
            // If no mod declarations, insert after doc comments
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
    lines.insert(mod_insert_idx, &mod_decl);

    // Find position to insert pub use (after other pub use declarations)
    let pub_use_decl = format!("pub use {}::{};", file_name, struct_name);
    let mut last_pub_use_idx = None;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("pub use ") {
            last_pub_use_idx = Some(i);
        }
    }

    // Insert pub use declaration
    match last_pub_use_idx {
        Some(idx) => {
            lines.insert(idx + 1, &pub_use_decl);
        }
        None => {
            // If no pub use declarations, add after mod declarations with empty line
            let mut insert_idx = mod_insert_idx + 1;
            // Skip past remaining mod declarations
            while insert_idx < lines.len() && lines[insert_idx].trim().starts_with("mod ") {
                insert_idx += 1;
            }
            // Add empty line if needed
            if insert_idx < lines.len() && !lines[insert_idx].is_empty() {
                lines.insert(insert_idx, "");
                insert_idx += 1;
            }
            lines.insert(insert_idx, &pub_use_decl);
        }
    }

    let new_content = lines.join("\n");
    fs::write(mod_file, new_content).map_err(|e| format!("Failed to write mod.rs: {}", e))?;

    Ok(())
}
