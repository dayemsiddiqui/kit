use console::style;
use std::fs;
use std::path::Path;

use crate::templates;

pub fn run(name: String) {
    // Convert to PascalCase for struct name
    let struct_name = to_pascal_case(&name);
    // Convert to snake_case for file name
    let file_name = to_snake_case(&name);

    // Validate the resulting names
    if !is_valid_identifier(&file_name) {
        eprintln!(
            "{} '{}' is not a valid request name",
            style("Error:").red().bold(),
            name
        );
        std::process::exit(1);
    }

    let requests_dir = Path::new("src/requests");
    let request_file = requests_dir.join(format!("{}.rs", file_name));
    let mod_file = requests_dir.join("mod.rs");

    // Create requests directory if it doesn't exist
    if !requests_dir.exists() {
        if let Err(e) = fs::create_dir_all(requests_dir) {
            eprintln!(
                "{} Failed to create requests directory: {}",
                style("Error:").red().bold(),
                e
            );
            std::process::exit(1);
        }
        println!(
            "{} Created src/requests directory",
            style("✓").green()
        );
    }

    // Check if request file already exists
    if request_file.exists() {
        eprintln!(
            "{} Request '{}' already exists at {}",
            style("Info:").yellow().bold(),
            struct_name,
            request_file.display()
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
                "{} Module '{}' is already declared in src/requests/mod.rs",
                style("Info:").yellow().bold(),
                file_name
            );
            std::process::exit(0);
        }
    }

    // Generate request file content
    let request_content = templates::request_template(&struct_name);

    // Write request file
    if let Err(e) = fs::write(&request_file, request_content) {
        eprintln!(
            "{} Failed to write request file: {}",
            style("Error:").red().bold(),
            e
        );
        std::process::exit(1);
    }
    println!(
        "{} Created {}",
        style("✓").green(),
        request_file.display()
    );

    // Update or create mod.rs
    if mod_file.exists() {
        if let Err(e) = update_mod_file(&mod_file, &file_name, &struct_name) {
            eprintln!(
                "{} Failed to update mod.rs: {}",
                style("Error:").red().bold(),
                e
            );
            std::process::exit(1);
        }
        println!("{} Updated src/requests/mod.rs", style("✓").green());
    } else {
        // Create mod.rs with pub mod and pub use
        let mod_content = format!(
            "pub mod {};\n\npub use {}::{};\n",
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
        println!("{} Created src/requests/mod.rs", style("✓").green());
    }

    println!();
    println!(
        "FormRequest {} created successfully!",
        style(&struct_name).cyan().bold()
    );
    println!();
    println!("Usage in a controller:");
    println!(
        "  {}",
        style(format!(
            "use crate::requests::{};",
            struct_name
        ))
        .dim()
    );
    println!();
    println!(
        "  {}",
        style("#[handler]").dim()
    );
    println!(
        "  {}",
        style(format!(
            "pub async fn store(form: {}) -> Response {{",
            struct_name
        ))
        .dim()
    );
    println!("      {}",
        style("// form is automatically validated - returns 422 if invalid").dim()
    );
    println!(
        "      {}",
        style("json_response!({ \"success\": true })").dim()
    );
    println!("  {}", style("}").dim());
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

    let mut lines: Vec<&str> = content.lines().collect();

    // Find positions for pub mod and pub use declarations
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

    // Insert pub mod declaration
    let mod_insert_idx = match last_pub_mod_idx {
        Some(idx) => idx + 1,
        None => 0,
    };
    lines.insert(mod_insert_idx, &pub_mod_decl);

    // Insert pub use declaration (after the pub mod section)
    let use_insert_idx = match last_pub_use_idx {
        Some(idx) => idx + 2, // +2 because we just inserted a line
        None => {
            // If no pub use declarations exist, add after all pub mod declarations
            // Find the last pub mod line again (accounting for the new line we added)
            let mut new_last_mod = mod_insert_idx;
            for (i, line) in lines.iter().enumerate() {
                if line.trim().starts_with("pub mod ") {
                    new_last_mod = i;
                }
            }
            new_last_mod + 1
        }
    };

    // Add empty line if needed and then the pub use
    if use_insert_idx < lines.len() && !lines[use_insert_idx - 1].is_empty() && !lines[use_insert_idx - 1].starts_with("pub use") {
        lines.insert(use_insert_idx, "");
        lines.insert(use_insert_idx + 1, &pub_use_decl);
    } else {
        lines.insert(use_insert_idx, &pub_use_decl);
    }

    let new_content = lines.join("\n");
    fs::write(mod_file, new_content).map_err(|e| format!("Failed to write mod.rs: {}", e))?;

    Ok(())
}
